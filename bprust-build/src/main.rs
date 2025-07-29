use std::{
    env::current_dir,
    fs::{self, File, read_to_string},
    path::Path,
};

use anyhow::Result;
use bprust_build::{BPDefinitions, codegen::generate_rust_code};

fn main() -> Result<()> {
    const JSON_PATH: &str = "../BPRust/blueprint_definitions.json";
    let json_path = current_dir()?.join(JSON_PATH);

    println!("{:?}", json_path);
    let string = read_to_string(&json_path)?;
    println!("read done");
    let json: BPDefinitions = serde_json::from_str(&string)?;
    let string = generate_rust_code(json, true)?;
    fs::write("out.rs", string)?;

    println!("done");
    Ok(())
}
