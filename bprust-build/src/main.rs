use std::{
    env::current_dir,
    fs::{self, File, read_to_string},
    path::Path,
};

use anyhow::Result;

fn main() -> Result<()> {
    const JSON_PATH: &str = "../BPRust/blueprint_definitions.json";
    let json_path = current_dir()?.join(JSON_PATH);

    println!("{:?}", json_path);
    let string = read_to_string(&json_path)?;
    println!("read done");
    let code = bprust_build::compile(&string, true)?;
    fs::write("out.rs", code)?;

    println!("done");
    Ok(())
}
