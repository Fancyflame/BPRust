use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use crate::{
    DefClass, DefFunction,
    codegen::{Codegen, ContentDefinition, gen_class::gen_functions::FunctionsCodeGen},
};

mod gen_functions;

impl<'a> Codegen<'a> {
    pub fn gen_class(&mut self, class: DefClass<'a>) {
        let class_safe_name = self
            .symbols
            .lookup_name(class.name)
            .unwrap()
            .safe_name
            .clone();

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
