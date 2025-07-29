use std::{collections::HashMap, ops::Not};

use anyhow::{Result, anyhow};

use crate::{
    DefStruct, PropertyType,
    codegen::{ContentDefinition, SymbolMap},
};

pub(super) fn define_struct_symbols<'a>(
    symbols: &mut SymbolMap<'a>,
    structs: &Vec<DefStruct<'a>>,
) -> Result<()> {
    let mut struct_table =
        HashMap::from_iter(structs.iter().map(|s| (s.id, StructState::Unresolved(s))));

    for struct_def in structs {
        if let StructState::Unresolved(_) = &struct_table[struct_def.id] {
            insert_struct_symbol(struct_def, &mut struct_table, symbols)?;
        }
    }

    Ok(())
}

enum StructState<'d, 'a> {
    Unresolved(&'d DefStruct<'a>),
    Resolved { contains_lifetime: bool },
}

fn insert_struct_symbol<'a>(
    struct_def: &DefStruct<'a>,
    struct_table: &mut HashMap<&str, StructState<'_, 'a>>,
    symbols: &mut SymbolMap<'a>,
) -> Result<bool> {
    let mut contains_lifetime = false;
    for member in &struct_def.members {
        contains_lifetime = match &member.prop_type {
            PropertyType::Enum(_) | PropertyType::Primitive(_) => false,
            PropertyType::Object(_) => true,
            &PropertyType::Struct(prop_struct_name) => match struct_table.get(prop_struct_name) {
                Some(struct_state) => match struct_state {
                    StructState::Resolved { contains_lifetime } => *contains_lifetime,
                    StructState::Unresolved(def) => {
                        insert_struct_symbol(def, struct_table, symbols)?
                    }
                },
                // struct not found. but it's not the time to throw error
                None => false,
            },
        };
        if contains_lifetime {
            break;
        }
    }

    *struct_table.get_mut(struct_def.id).unwrap() = StructState::Resolved { contains_lifetime };
    symbols.resolve_insert(
        struct_def.id,
        struct_def.name,
        ContentDefinition::Struct { contains_lifetime },
    );
    Ok(contains_lifetime)
}
