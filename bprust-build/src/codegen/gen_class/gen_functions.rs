use crate::{
    DefClass, DefFunction, DefProperty, EPropertyFlag,
    codegen::{SymbolMap, resolve_property::ResolvedTypeOfProperty},
};
use anyhow::Result;
use case::CaseExt;
use quote::format_ident;
use syn::Ident;

use crate::codegen::SafeNameCast;

pub(super) struct FunctionsCodeGen<'r> {
    symbols: &'r SymbolMap<'r>,
    safe_function_names: SafeNameCast,
    safe_param_names: SafeNameCast,
    params: Vec<FnParam<'r>>,
    out_param_contains_lifetime: bool,
    out_param_count: OutParamCount,
}

impl<'r> FunctionsCodeGen<'r> {
    pub fn new(symbols: &'r SymbolMap<'r>) -> Self {
        Self {
            symbols,
            safe_function_names: SafeNameCast::new(),
            safe_param_names: SafeNameCast::new(),
            params: Vec::new(),
            out_param_contains_lifetime: false,
            out_param_count: OutParamCount::Zero,
        }
    }

    pub fn generate_functions(
        &mut self,
        def_class: &DefClass<'r>,
    ) -> Result<Vec<FunctionInst<'r>>> {
        let mut output = Vec::with_capacity(def_class.functions.len());
        self.start_define_class();

        for &DefFunction {
            id,
            name: fn_name,
            rust_override,
            ref params,
        } in &def_class.functions
        {
            self.start_define_function();

            let safe_func_name = self.define_name(fn_name);
            for (index, param) in params.iter().enumerate() {
                self.define_param(index, param)?;
            }

            let return_type = self.get_return_type(&safe_func_name);
            output.push(FunctionInst {
                fn_name: safe_func_name,
                id,
                params: std::mem::take(&mut self.params),
                return_type,
            });
        }

        Ok(output)
    }

    fn start_define_class(&mut self) {
        self.safe_function_names.clear();
    }

    fn start_define_function(&mut self) {
        self.safe_param_names.clear();
        self.params.clear();
        self.out_param_contains_lifetime = false;
        self.out_param_count = OutParamCount::Zero;
    }

    fn define_name(&mut self, fn_name: &str) -> Ident {
        self.safe_function_names.to_safe_name(fn_name)
    }

    fn define_param(&mut self, index: usize, param: &DefProperty) -> Result<()> {
        let is_output = param.flags & (EPropertyFlag::OutParm | EPropertyFlag::ReturnParm) != 0;
        let param_name = self.safe_param_names.to_safe_name(&param.name.to_snake());
        let param_type = self.symbols.get_type_of_property(&param.prop_type)?;

        if is_output {
            self.out_param_contains_lifetime |= param_type.contains_lifetime();
            self.out_param_count = match self.out_param_count {
                OutParamCount::Zero => OutParamCount::One { index },
                _ => OutParamCount::Many,
            };
        };

        self.params.push(FnParam {
            name: param_name,
            ty: param_type,
            is_out: is_output,
        });

        Ok(())
    }

    fn get_return_type(&mut self, fn_name: &Ident) -> ReturnType<'r> {
        match self.out_param_count {
            OutParamCount::Zero => ReturnType::None,
            OutParamCount::One { index } => {
                let p = &self.params[index];
                ReturnType::Single(p.name.clone(), p.ty)
            }
            OutParamCount::Many => ReturnType::Multiple(ReturnStruct {
                struct_name: format_ident!("BPRustReturnTypeOf{fn_name}"),
                contains_lifetime: self.out_param_contains_lifetime,
            }),
        }
    }
}

#[derive(Clone, Copy)]
enum OutParamCount {
    Zero,
    One { index: usize },
    Many,
}

pub struct FunctionInst<'r> {
    pub fn_name: Ident,
    pub id: &'r str,
    pub params: Vec<FnParam<'r>>,
    pub return_type: ReturnType<'r>,
}

pub enum ReturnType<'r> {
    None,
    Single(Ident, ResolvedTypeOfProperty<'r>),
    Multiple(ReturnStruct),
}

pub struct ReturnStruct {
    pub struct_name: Ident,
    pub contains_lifetime: bool,
}

pub struct FnParam<'r> {
    pub name: Ident,
    pub ty: ResolvedTypeOfProperty<'r>,
    pub is_out: bool,
}
