use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{
    PropPrimitiveType, PropertyType,
    codegen::{ContentDefinition, LinkedContent, SymbolMap},
};

impl<'a> SymbolMap<'a> {
    pub(super) fn get_type_of_property(
        &self,
        prop: &PropertyType,
    ) -> Result<ResolvedTypeOfProperty> {
        match prop {
            PropertyType::Enum(sym) | PropertyType::Object(sym) | PropertyType::Struct(sym) => {
                self.lookup_name(sym).map(ResolvedTypeOfProperty::Symbol)
            }
            PropertyType::Primitive(prim) => Ok(ResolvedTypeOfProperty::Primitive(*prim)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ResolvedTypeOfProperty<'a> {
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
        }
    }

    pub fn type_tokens(&self, explict_lifetime: bool) -> TokenStream {
        let linked_content = match self {
            Self::Primitive(prim) => return prim_to_tokens(*prim),
            Self::Symbol(lc) => lc,
        };
        let symbol_name = &linked_content.safe_name;

        match linked_content.def {
            ContentDefinition::Enum => symbol_name.to_token_stream(),
            ContentDefinition::Class => {
                if explict_lifetime {
                    quote! { &'obj #symbol_name }
                } else {
                    quote! { &#symbol_name }
                }
            }
            ContentDefinition::Struct { contains_lifetime } => {
                let lifetime_tokens = (contains_lifetime && explict_lifetime).then(|| {
                    quote! {<'obj>}
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
