use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{
    PropPrimitiveType, PropertyType,
    codegen::{ContentDefinition, LinkedContent, SymbolMap},
};

macro_rules! match_prim {
    ($var:ident => {$($Ident:ident $Ty:ty,)*}) => {
        match $var {
            $(PropPrimitiveType::$Ident => quote! { $Ty },)*
        }
    };
}

impl<'a> SymbolMap<'a> {
    pub(super) fn get_type_of_property(
        &self,
        prop: &PropertyType,
        name_lifetime: bool,
    ) -> Result<TokenStream> {
        let tokens = match &prop {
            PropertyType::Enum(sym) => self.lookup_name(sym)?.safe_name.to_token_stream(),
            PropertyType::Object(sym) => {
                let object_name = &self.lookup_name(sym)?.safe_name;
                if name_lifetime {
                    quote! { &'obj #object_name }
                } else {
                    quote! { &#object_name }
                }
            }
            &PropertyType::Struct(stru) => {
                let LinkedContent {
                    safe_name,
                    def: ContentDefinition::Struct { contains_lifetime },
                } = self.lookup_name(stru)?
                else {
                    unreachable!();
                };

                let lifetime_tokens = (*contains_lifetime && name_lifetime).then(|| {
                    quote! {<'obj>}
                });

                quote! {
                    #safe_name #lifetime_tokens
                }
            }
            PropertyType::Primitive(prim) => match_prim!(prim => {
                Name   bprust_sys::FName,
                Str    bprust_sys::FString,
                Text   bprust_sys::FText,
                Bool   bool,
                Byte   u8,
                Int    i32,
                Int64  i64,
                Float  f32,
                Double f64,
            }),
        };
        Ok(tokens)
    }
}
