use anyhow::Result;
use proc_macro2::TokenStream;

use crate::{
    codegen::{Codegen, LifetimeConst},
    json_definitions::DefStruct,
};

impl Codegen<'_> {
    pub fn gen_struct(&self, def_struct: &DefStruct) -> Result<TokenStream> {
        // def_struct.members.iter().map(|member| {
        //     self.symbols
        //         .get_type_of_property(&member.prop_type)
        //         .map(|ty| ty.type_tokens(LifetimeConst::DefStruct))
        // });
    }
}
