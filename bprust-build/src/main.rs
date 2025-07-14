use std::{
    env::current_dir,
    fs::{File, read_to_string},
    path::Path,
};

use anyhow::Result;
use bprust_build::BPDefinitions;

fn main() -> Result<()> {
    const JSON_PATH: &str = "../BPRust/blueprint_definitions.json";
    let json_path = current_dir()?.join(JSON_PATH);

    println!("{:?}", json_path);
    let string = read_to_string(&json_path)?;
    println!("read done");
    let json: BPDefinitions = serde_json::from_str(&string)?;
    println!("done");
    Ok(())
}
