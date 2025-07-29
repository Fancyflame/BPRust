use std::collections::HashMap;

use serde::Deserialize;

#[path = "property_flag.rs"]
#[allow(non_snake_case)]
mod EPropertyFlag;

pub mod codegen;

#[derive(Deserialize)]
pub struct BPDefinitions<'a> {
    #[serde(borrow)]
    classes: Vec<DefClass<'a>>,
    structs: Vec<DefStruct<'a>>,
    enums: Vec<DefEnum<'a>>,
    basic_types: HashMap<&'a str, DefBasic>,
}

#[derive(Deserialize)]
struct DefClass<'a> {
    name: &'a str,
    id: &'a str,
    #[serde(rename = "super")]
    super_class: String,
    properties: Vec<DefProperty<'a>>,
    functions: Vec<DefFunction<'a>>,
}

#[derive(Deserialize)]
struct DefStruct<'a> {
    name: &'a str,
    id: &'a str,
    members: Vec<DefProperty<'a>>,
}

#[derive(Deserialize)]
struct DefBasic {
    size: u64,
    align: u64,
}

#[derive(Deserialize)]
struct DefEnum<'a> {
    id: &'a str,
    variants: HashMap<&'a str, i64>,
}

#[derive(Deserialize)]
struct DefFunction<'a> {
    id: &'a str,
    name: &'a str,
    #[serde(rename = "override", default)]
    rust_override: bool,
    params: Vec<DefProperty<'a>>,
}

#[derive(Deserialize)]
struct DefProperty<'a> {
    name: &'a str,
    #[serde(flatten)]
    prop_type: PropertyType<'a>,
    flags: i64,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "property", content = "type_info")]
enum PropertyType<'a> {
    Primitive(PropPrimitiveType),
    Object(&'a str),
    Struct(&'a str),
    Enum(&'a str),
}

#[derive(Clone, Copy, Deserialize)]
enum PropPrimitiveType {
    Name,
    Str,
    Text,
    Bool,
    Byte,
    Int,
    Int64,
    Float,
    Double,
}
