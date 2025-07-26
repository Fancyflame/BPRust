use crate::{
    DefClass, DefFunction, DefProperty, EPropertyFlag,
    codegen::{ContentDefinition, SymbolMap},
};
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::Ident;

use crate::codegen::SafeNameCast;

#[derive(Default)]
pub(super) struct FunctionsCodeGen {
    safe_function_names: SafeNameCast,
    safe_param_names: SafeNameCast,
    input_params: Vec<(Ident, TokenStream)>,
    output_params: Vec<(Ident, TokenStream)>,
    out_param_contains_lifetime: bool,
}

impl FunctionsCodeGen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_functions<'a>(
        &mut self,
        def_class: &DefClass<'a>,
        symbols: &SymbolMap,
    ) -> Result<Vec<FunctionInst<'a>>> {
        let mut output = Vec::with_capacity(def_class.functions.len());
        self.start_define_class();

        for &DefFunction {
            id,
            name: fn_name,
            rust_override,
            ref params,
        } in &def_class.functions
        {
            self.start_define_functions();

            let safe_func_name = self.define_name(fn_name);
            for param in params {
                self.define_param(param, symbols)?;
            }

            let return_type = self.get_return_type(&safe_func_name);
            output.push(FunctionInst {
                fn_name: safe_func_name,
                ufunction_name: id,
                in_params: std::mem::take(&mut self.input_params),
                return_type,
            });
        }

        Ok(output)
    }

    fn start_define_class(&mut self) {
        self.safe_function_names.clear();
    }

    fn start_define_functions(&mut self) {
        self.safe_param_names.clear();
        self.input_params.clear();
        self.output_params.clear();
        self.out_param_contains_lifetime = false;
    }

    fn define_name(&mut self, fn_name: &str) -> Ident {
        self.safe_function_names.to_safe_name(fn_name)
    }

    fn define_param(&mut self, param: &DefProperty, symbols: &SymbolMap) -> Result<()> {
        let is_output = param.flags & (EPropertyFlag::OutParm | EPropertyFlag::ReturnParm) != 0;
        let param_name = self.safe_param_names.to_safe_name(param.name);
        let param_type = symbols.get_type_of_property(&param.prop_type)?;

        if is_output {
            self.out_param_contains_lifetime |= param_type.contains_lifetime();
            self.output_params
                .push((param_name, param_type.type_tokens(true)));
        } else {
            self.input_params
                .push((param_name, param_type.type_tokens(false)));
        };

        Ok(())
    }

    fn get_return_type(&mut self, fn_name: &Ident) -> ReturnType {
        match self.output_params.len() {
            0 => ReturnType::None,
            1 => ReturnType::Single(self.output_params.pop().unwrap().1),
            _ => ReturnType::Multiple(ReturnStruct {
                struct_name: format_ident!("ReturnTypeOf{fn_name}"),
                contains_lifetime: self.out_param_contains_lifetime,
                out_params: std::mem::take(&mut self.output_params),
            }),
        }
    }
}

pub struct FunctionInst<'a> {
    pub fn_name: Ident,
    pub ufunction_name: &'a str,
    pub in_params: Vec<(Ident, TokenStream)>,
    pub return_type: ReturnType,
}

pub enum ReturnType {
    None,
    Single(TokenStream),
    Multiple(ReturnStruct),
}

pub struct ReturnStruct {
    pub struct_name: Ident,
    pub contains_lifetime: bool,
    pub out_params: Vec<(Ident, TokenStream)>,
}
