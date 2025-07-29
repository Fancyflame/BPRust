use std::collections::{HashMap, hash_map::Entry};

use anyhow::{Result, anyhow};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use self::lifetime_const::*;
use crate::{BPDefinitions, DefStruct, PropertyType, codegen::safe_name::SafeNameCast};

mod define_struct;
mod gen_class;
mod lifetime_const;
mod resolve_property;
mod safe_name;

pub fn generate_rust_code(definitions: BPDefinitions, prettify: bool) -> Result<String> {
    let mut codegen = Codegen::new();
    codegen.define_symbols(&definitions)?;
    let tokens = codegen.generate_code(&definitions)?;

    let mut token_string = tokens.to_string();
    if prettify {
        let syn_file = syn::parse_file(&token_string)?;
        token_string = prettyplease::unparse(&syn_file);
    }

    Ok(token_string)
}

enum ContentDefinition {
    Class,
    Struct { contains_lifetime: bool },
    Enum,
}

pub(crate) struct Codegen<'a> {
    symbols: SymbolMap<'a>,
}

impl<'a> Codegen<'a> {
    pub fn new() -> Self {
        Self {
            symbols: SymbolMap {
                symbols: HashMap::new(),
                safe_name: SafeNameCast::new(),
            },
        }
    }

    fn define_symbols(&mut self, definitions: &BPDefinitions<'a>) -> Result<()> {
        for class in &definitions.classes {
            self.symbols
                .resolve_insert(class.id, class.name, ContentDefinition::Class);
        }
        for enum_def in &definitions.enums {
            self.symbols
                .resolve_insert(enum_def.id, enum_def.id, ContentDefinition::Enum);
        }
        define_struct::define_struct_symbols(&mut self.symbols, &definitions.structs)?;
        Ok(())
    }

    fn generate_code(&mut self, definitions: &BPDefinitions<'a>) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();

        for class in &definitions.classes {
            self.gen_class(&mut tokens, class)?;
        }

        Ok(tokens)
    }
}

struct SymbolMap<'a> {
    symbols: HashMap<&'a str, LinkedContent>,
    safe_name: SafeNameCast,
}

struct LinkedContent {
    safe_name: Ident,
    def: ContentDefinition,
}

impl<'a> SymbolMap<'a> {
    fn resolve_insert<'r>(&'r mut self, id: &'a str, name: &str, insert: ContentDefinition) {
        let occupied = self.symbols.insert(
            id,
            LinkedContent {
                safe_name: self.safe_name.to_safe_name(name),
                def: insert,
            },
        );

        if occupied.is_some() {
            println!("cargo:warning=symbol `{name}` is already defined");
        }
        // assert!(occupied.is_none(), "symbol `{name}` is already defined");
    }

    fn lookup_name<'r>(&'r self, name: &str) -> Option<&'r LinkedContent> {
        self.symbols.get(name)
        //.ok_or_else(|| anyhow!("symbol `{name}` is not found"))
    }

    fn contains(&self, id: &str) -> bool {
        self.symbols.contains_key(id)
    }
}
