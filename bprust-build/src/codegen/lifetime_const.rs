use proc_macro2::TokenStream;

use quote::{ToTokens, quote};

pub(super) const INPUT_LIFETIME: LifetimeConst = LifetimeConst::Input;
pub(super) const OUTPUT_LIFETIME: LifetimeConst = LifetimeConst::Output;

#[derive(Clone, Copy)]
pub(super) enum LifetimeConst {
    Anonymous,
    Input,
    Output,
    DefStruct,
}

impl ToTokens for LifetimeConst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Anonymous => quote! {'_},
            Self::Input => quote! {'input},
            Self::Output => quote! {'output},
            Self::DefStruct => quote! {'obj},
        }
        .to_tokens(tokens)
    }
}
