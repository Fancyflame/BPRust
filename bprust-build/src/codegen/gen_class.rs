use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use crate::{
    DefClass, DefFunction, DefProperty, EPropertyFlag,
    codegen::{Codegen, ContentDefinition, SafeNameCast, SymbolMap},
};

impl<'a> Codegen<'a> {
    pub fn gen_class(&mut self, class: DefClass<'a>) {
        let class_safe_name = self.symbols.resolve_name(class.name).safe_name.clone();
        let ccg = ClassCodeGen {
            class,
            fn_return_structs_module_name: format_ident!(
                "bprust_return_types_of_{class_safe_name}"
            ),
            fn_codegen: FunctionsCodeGen::new(),
            class_safe_name,
            outside: self,
        };

        // quote! {
        //     pub struct #class_safe_name(());

        //     impl #class_safe_name {

        //     }

        //     pub mod #module_name {

        //     }
        // };
    }
}

struct ClassCodeGen<'c, 'a> {
    outside: &'c mut Codegen<'a>,
    class: DefClass<'a>,
    class_safe_name: Ident,
    fn_return_structs_module_name: Ident,
    fn_codegen: FunctionsCodeGen,
}

impl<'a> ClassCodeGen<'_, 'a> {
    fn generate_functions(&mut self) -> Result<TokenStream> {
        let mut fn_impl_tokens = TokenStream::new();
        let mut ret_struct_tokens = TokenStream::new();
        let mut fn_codegen = &mut self.fn_codegen;

        fn_codegen.start_define_class();
        for &DefFunction {
            id,
            name: fn_name,
            rust_override,
            ref params,
        } in &self.class.functions
        {
            fn_codegen.start_define_functions();

            let safe_func_name = fn_codegen.define_name(fn_name);
            for param in params {
                fn_codegen.define_param(param, &self.outside.symbols)?;
            }

            let (ret_ty, ret_struct) = fn_codegen.get_return_type(fn_name);
            ret_struct.to_tokens(&mut ret_struct_tokens);
        }

        fn_impl_tokens
    }
}

enum ReturnType<'a> {
    None,
    Single(&'a TokenStream),
    Many,
}

#[derive(Default)]
struct FunctionsCodeGen {
    safe_function_names: SafeNameCast,
    safe_param_names: SafeNameCast,
    input_params: Vec<(Ident, TokenStream)>,
    output_params: Vec<(Ident, TokenStream)>,
    out_param_contains_lifetime: bool,
}

impl FunctionsCodeGen {
    fn new() -> Self {
        Self::default()
    }

    fn start_define_class(&mut self) {
        self.safe_function_names.clear_registered();
    }

    fn start_define_functions(&mut self) {
        self.safe_param_names.clear_registered();
        self.input_params.clear();
        self.output_params.clear();
        self.out_param_contains_lifetime = false;
    }

    fn define_name(&mut self, fn_name: &str) -> Ident {
        self.safe_function_names.to_safe_name(fn_name)
    }

    fn define_param(&mut self, param: &DefProperty, symbols: &SymbolMap) -> Result<()> {
        let is_output = param.flags & (EPropertyFlag::OutParm | EPropertyFlag::ReturnParm) != 0;

        let dst_vec = if is_output {
            self.out_param_contains_lifetime |= matches!(
                &symbols.lookup_name(param.name)?.def,
                ContentDefinition::Struct {
                    contains_lifetime: true
                }
            );
            &mut self.output_params
        } else {
            &mut self.input_params
        };

        dst_vec.push((
            self.safe_param_names.to_safe_name(param.name),
            symbols.get_type_of_property(&param.property, is_output)?,
        ));

        Ok(())
    }

    fn get_return_type(&self, fn_name: &str) -> (TokenStream, TokenStream) {
        match &*self.output_params {
            [] => return (quote! {}, quote! {}),
            [(_, one)] => return (quote! {-> #one}, quote! {}),
            _ => {}
        }

        let lt_param = self.out_param_contains_lifetime.then(|| quote! {'obj});
        let struct_name = format_ident!("ReturnTypeOf{fn_name}");
        let fields = self
            .output_params
            .iter()
            .map(|(name, ty)| quote! {#name: #ty});

        (
            quote! {-> #struct_name #lt_param},
            quote! {
                pub struct #struct_name #lt_param {
                    #(#fields)*
                }
            },
        )
    }
}
