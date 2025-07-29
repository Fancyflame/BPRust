use std::ffi::{CStr, CString};

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use crate::{
    DefClass, DefFunction,
    codegen::{
        Codegen, ContentDefinition, INPUT_LIFETIME, OUTPUT_LIFETIME,
        gen_class::gen_functions::{
            FnParam, FunctionInst, FunctionsCodeGen, ReturnStruct, ReturnType,
        },
        lifetime_const::LifetimeConst,
    },
};

mod gen_functions;

impl<'a> Codegen<'a> {
    pub fn gen_class(&mut self, tokens: &mut TokenStream, class: &DefClass<'a>) -> Result<()> {
        let class_name = &self.symbols.lookup_name(class.id).unwrap().safe_name;
        let functions = FunctionsCodeGen::new(&self.symbols).generate_functions(&class)?;

        let codegen = ClassCodeGen {
            class_name,
            id: class.id,
            fn_return_structs_module_name: format_ident!("bprust_return_types_of_{class_name}"),
            functions,
        };

        codegen.to_tokens().to_tokens(tokens);
        Ok(())
    }
}

struct ClassCodeGen<'r> {
    class_name: &'r Ident,
    id: &'r str,
    fn_return_structs_module_name: Ident,
    functions: Vec<FunctionInst<'r>>,
}

impl ClassCodeGen<'_> {
    fn to_tokens(&self) -> TokenStream {
        let Self {
            class_name,
            id,
            fn_return_structs_module_name,
            functions,
        } = self;

        let function_definitions = functions
            .iter()
            .map(|f| generate_function(f, fn_return_structs_module_name));

        let function_return_module = {
            let mut structs = functions
                .iter()
                .filter_map(generate_function_return_struct)
                .peekable();

            structs.peek().is_some().then(|| {
                quote! {
                    pub mod #fn_return_structs_module_name {
                        #(#structs)*
                    }
                }
            })
        };

        quote! {
            pub struct #class_name(());
            #function_return_module

            impl #class_name {
                #(#function_definitions)*
            }
        }
    }
}

fn generate_function(func: &FunctionInst, ret_mod: &Ident) -> TokenStream {
    let FunctionInst {
        fn_name,
        id,
        params,
        return_type,
    } = func;

    let arguments = params.iter().filter_map(|FnParam { name, ty, is_out }| {
        if *is_out {
            None
        } else {
            let ty = ty.type_tokens(LifetimeConst::Anonymous);
            Some(quote! { #name: #ty, })
        }
    });

    let return_type = match return_type {
        ReturnType::None => quote! {},
        ReturnType::Single(_, ty) => {
            let ty = ty.type_tokens(LifetimeConst::Output);
            quote! { -> #ty }
        }
        ReturnType::Multiple(ReturnStruct {
            struct_name,
            contains_lifetime,
            ..
        }) => {
            let lifetime = contains_lifetime.then(|| quote! {<#OUTPUT_LIFETIME>});
            quote! {
                -> #ret_mod::#struct_name #lifetime
            }
        }
    };

    let param_struct_fields = params.iter().map(|FnParam { name, ty, is_out }| {
        let ty = ty.type_tokens(if *is_out {
            LifetimeConst::Output
        } else {
            LifetimeConst::Input
        });
        quote! { #name: ::core::mem::MaybeUninit<#ty>, }
    });

    let input_param_names: Vec<&Ident> = params
        .iter()
        .filter_map(|p| (!p.is_out).then_some(&p.name))
        .collect();

    let output_param_names: Vec<&Ident> = params
        .iter()
        .filter_map(|p| p.is_out.then_some(&p.name))
        .collect();

    let Ok(ufunc_name) = CString::new(*id) else {
        panic!("cannot generate function name `{id}`")
    };

    let return_expr = match &func.return_type {
        ReturnType::None => quote! {},
        ReturnType::Single(ident, _) => quote! {
            #ident
        },
        ReturnType::Multiple(ReturnStruct { struct_name, .. }) => quote! {
            #ret_mod::#struct_name {
                #(#output_param_names: params.#output_param_names.assume_init(),)*
            }
        },
    };

    quote! {
        pub fn #fn_name<#OUTPUT_LIFETIME>(&#OUTPUT_LIFETIME self, #(#arguments)*) #return_type {
            #[repr(C)]
            struct __BPRustFunctionParameters<#INPUT_LIFETIME, #OUTPUT_LIFETIME> {
                _capture_lifetime: ::core::marker::PhantomData<(
                    &#INPUT_LIFETIME (),
                    &#OUTPUT_LIFETIME (),
                )>,
                #(#param_struct_fields)*
            }

            let mut params = __BPRustFunctionParameters {
                #(#input_param_names: ::core::mem::MaybeUninit::new(#input_param_names),)*
                #(#output_param_names: ::core::mem::MaybeUninit::uninit(),)*
            };

            unsafe {
                bprust_sys::cpp_import::cpp_get().process_event(
                    self as *const Self as _,
                    #ufunc_name,
                    &mut params,
                );
                #return_expr
            }
        }
    }
}

fn generate_function_return_struct(func: &FunctionInst) -> Option<TokenStream> {
    let ReturnType::Multiple(ReturnStruct {
        struct_name,
        contains_lifetime,
    }) = &func.return_type
    else {
        return None;
    };

    let out_params = func.params.iter().filter_map(|func| {
        if !func.is_out {
            return None;
        }
        let name = &func.name;
        let ty = func.ty.type_tokens(LifetimeConst::Output);
        Some(quote! {
            #name: #ty,
        })
    });

    let lifetime_generic = contains_lifetime.then_some(LifetimeConst::Output);

    Some(quote! {
        pub struct #struct_name<#lifetime_generic> {
            #(#out_params)*
        }
    })
}
