use std::collections::HashMap;

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
        HashMap::from_iter(structs.iter().map(|s| (s.name, StructState::Unresolved(s))));

    for struct_def in structs {
        insert_struct_symbol(struct_def, &mut struct_table, symbols)?;
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
        contains_lifetime = match &member.property {
            PropertyType::Enum(_) | PropertyType::Primitive(_) => false,
            PropertyType::Object(_) => true,
            &PropertyType::Struct(prop_struct_name) => {
                let struct_state = struct_table
                    .get_mut(prop_struct_name)
                    .ok_or_else(|| anyhow!("struct `{prop_struct_name}` is not defined"))?;

                match struct_state {
                    StructState::Resolved {
                        contains_lifetime: need_lifetime,
                    } => *need_lifetime,
                    &mut StructState::Unresolved(def) => {
                        insert_struct_symbol(def, struct_table, symbols)?
                    }
                }
            }
        };
        if contains_lifetime {
            break;
        }
    }

    *struct_table.get_mut(struct_def.name).unwrap() = StructState::Resolved { contains_lifetime };
    symbols.resolve_name(struct_def.name).def = ContentDefinition::Struct { contains_lifetime };
    Ok(contains_lifetime)
}
