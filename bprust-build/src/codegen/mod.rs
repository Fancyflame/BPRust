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
    Undefined,
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
            self.symbols.resolve_name(class.name).def = ContentDefinition::Class;
        }
        for enum_def in &definitions.enums {
            self.symbols.resolve_name(enum_def.name).def = ContentDefinition::Enum;
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
    fn resolve_name<'b>(&'b mut self, name: &'a str) -> &'b mut LinkedContent {
        match self.symbols.entry(name) {
            Entry::Occupied(occ) => occ.into_mut(),
            Entry::Vacant(vac) => vac.insert(LinkedContent {
                safe_name: self.safe_name.to_safe_name(name),
                def: ContentDefinition::Undefined,
            }),
        }
    }

    fn lookup_name(&self, name: &str) -> Result<&LinkedContent> {
        self.symbols
            .get(name)
            .ok_or_else(|| anyhow!("symbol `{name}` is not found"))
    }

    fn resolve_names<'b, const N: usize>(&'b mut self, names: [&'a str; N]) -> [&'b Ident; N] {
        for name in names {
            self.resolve_name(name);
        }
        names.map(|name| &self.symbols[name].safe_name)
    }
}
