use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct BPDefinitions<'a> {
    #[serde(borrow)]
    pub classes: Vec<DefClass<'a>>,
    pub structs: Vec<DefStruct<'a>>,
    pub enums: Vec<DefEnum<'a>>,
    pub basic_types: HashMap<&'a str, DefBasic>,
}

#[derive(Deserialize)]
pub(crate) struct DefClass<'a> {
    pub name: &'a str,
    pub id: &'a str,
    #[serde(rename = "super")]
    pub super_class: String,
    pub properties: Vec<DefProperty<'a>>,
    pub functions: Vec<DefFunction<'a>>,
}

#[derive(Deserialize)]
pub(crate) struct DefStruct<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub members: Vec<DefProperty<'a>>,
}

#[derive(Deserialize)]
pub(crate) struct DefBasic {
    pub size: u64,
    pub align: u64,
}

#[derive(Deserialize)]
pub(crate) struct DefEnum<'a> {
    pub id: &'a str,
    pub variants: HashMap<&'a str, i64>,
}

#[derive(Deserialize)]
pub(crate) struct DefFunction<'a> {
    pub id: &'a str,
    pub name: &'a str,
    #[serde(rename = "override", default)]
    pub rust_override: bool,
    pub params: Vec<DefProperty<'a>>,
}

#[derive(Deserialize)]
pub(crate) struct DefProperty<'a> {
    pub name: &'a str,
    #[serde(flatten)]
    pub prop_type: PropertyType<'a>,
    pub flags: i64,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "property", content = "type_info")]
pub(crate) enum PropertyType<'a> {
    Primitive(PropPrimitiveType),
    Object(&'a str),
    Struct(&'a str),
    Enum(&'a str),
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum PropPrimitiveType {
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
