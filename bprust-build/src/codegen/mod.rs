use std::collections::{HashMap, hash_map::Entry};

use anyhow::{Result, anyhow};
use proc_macro2::{Ident, TokenStream};

use crate::{BPDefinitions, DefStruct, PropertyType, codegen::safe_name::SafeNameCast};

mod define_struct;
mod gen_class;
mod reg_property;
mod safe_name;

pub fn generate_rust_code(definitions: BPDefinitions) -> Result<TokenStream> {
    let mut codegen = Codegen::new();
    codegen.define_symbols(definitions)?;
    Ok(codegen.tokens)
}

enum ContentDefinition {
    Class,
    Struct { contains_lifetime: bool },
    Enum,
}

pub(crate) struct Codegen<'a> {
    symbols: SymbolMap<'a>,
    tokens: TokenStream,
}

impl<'a> Codegen<'a> {
    pub fn new() -> Self {
        Self {
            symbols: SymbolMap {
                symbols: HashMap::new(),
                safe_name: SafeNameCast::new(),
            },
            tokens: TokenStream::new(),
        }
    }

    pub fn define_symbols(&mut self, definitions: BPDefinitions<'a>) -> Result<()> {
        for class in &definitions.classes {
            self.symbols
                .resolve_insert(class.name, ContentDefinition::Class);
        }
        for enum_def in &definitions.enums {
            self.symbols
                .resolve_insert(enum_def.name, ContentDefinition::Enum);
        }
        define_struct::define_struct_symbols(&mut self.symbols, &definitions.structs)?;
        Ok(())
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
    fn resolve_insert<'r>(&'r mut self, name: &'a str, insert: ContentDefinition) {
        let occupied = self.symbols.insert(
            name,
            LinkedContent {
                safe_name: self.safe_name.to_safe_name(name),
                def: insert,
            },
        );
        assert!(occupied.is_none(), "this symbol is already defined");
    }

    fn lookup_name<'r>(&'r self, name: &str) -> Result<&'r LinkedContent> {
        self.symbols
            .get(name)
            .ok_or_else(|| anyhow!("symbol `{name}` is not found"))
    }
}
