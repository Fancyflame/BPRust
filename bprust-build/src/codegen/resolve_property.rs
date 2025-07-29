use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{
    PropPrimitiveType, PropertyType,
    codegen::{ContentDefinition, LifetimeConst, LinkedContent, SymbolMap},
};

impl<'a> SymbolMap<'a> {
    pub(super) fn get_type_of_property(
        &self,
        prop: &PropertyType,
    ) -> Result<ResolvedTypeOfProperty> {
        match prop {
            PropertyType::Enum(sym) | PropertyType::Object(sym) | PropertyType::Struct(sym) => {
                match self.lookup_name(sym) {
                    Some(content) => Ok(ResolvedTypeOfProperty::Symbol(content)),
                    None => Ok(ResolvedTypeOfProperty::Undefined),
                }
            }
            PropertyType::Primitive(prim) => Ok(ResolvedTypeOfProperty::Primitive(*prim)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ResolvedTypeOfProperty<'a> {
    Undefined,
    Primitive(PropPrimitiveType),
    Symbol(&'a LinkedContent),
}

impl ResolvedTypeOfProperty<'_> {
    pub fn contains_lifetime(&self) -> bool {
        match self {
            Self::Symbol(lc) => match lc.def {
                ContentDefinition::Class => true,
                ContentDefinition::Enum => false,
                ContentDefinition::Struct { contains_lifetime } => contains_lifetime,
            },
            Self::Primitive(_) => false,
            Self::Undefined => false,
        }
    }

    pub fn type_tokens(&self, lifetime: LifetimeConst) -> TokenStream {
        let linked_content = match self {
            Self::Primitive(prim) => return prim_to_tokens(*prim),
            Self::Undefined => return quote! {!},
            Self::Symbol(lc) => lc,
        };
        let symbol_name = &linked_content.safe_name;

        match linked_content.def {
            ContentDefinition::Enum => symbol_name.to_token_stream(),
            ContentDefinition::Class => {
                quote! { &#lifetime #symbol_name }
            }
            ContentDefinition::Struct { contains_lifetime } => {
                let lifetime_tokens = contains_lifetime.then(|| {
                    quote! {<#lifetime>}
                });
                quote! {
                    #symbol_name #lifetime_tokens
                }
            }
        }
    }
}

fn prim_to_tokens(prim: PropPrimitiveType) -> TokenStream {
    macro_rules! match_prim {
        ($var:ident => {$($Ident:ident $Ty:ty,)*}) => {
            match $var {
                $(PropPrimitiveType::$Ident => quote! { $Ty },)*
            }
        };
    }

    match_prim!(prim => {
        Name   bprust_sys::FName,
        Str    bprust_sys::FString,
        Text   bprust_sys::FText,
        Bool   bool,
        Byte   u8,
        Int    i32,
        Int64  i64,
        Float  f32,
        Double f64,
    })
}
