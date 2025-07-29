use std::{
    env,
    fs::{create_dir_all, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use json_definitions::*;

use crate::codegen::generate_rust_code;

#[path = "property_flag.rs"]
#[allow(non_snake_case)]
mod EPropertyFlag;
mod codegen;
mod json_definitions;

pub fn compile(json: &str, code_prettify: bool) -> Result<String> {
    let def: BPDefinitions = serde_json::from_str(json)?;
    generate_rust_code(def, code_prettify)
}

pub fn build(json_path: impl AsRef<Path>, file_path: Option<&Path>) {
    let file = match read_to_string(json_path.as_ref()) {
        Ok(file) => file,
        Err(err) => {
            panic!("cannot read file `{}`: {err}", json_path.as_ref().display());
        }
    };

    let code = compile(&file, true).expect("compile error");

    let mut out_file = PathBuf::new();
    if let Ok(dir) = env::var("OUT_DIR") {
        out_file.push(dir);
    }
    out_file.push("bprust-build-result");
    if let Some(file_path) = file_path {
        out_file.push(file_path);
    } else {
        out_file.push("generated.rs");
    }

    if let Some(parent_dir) = out_file.parent() {
        if let Err(err) = create_dir_all(parent_dir) {
            panic!("cannot create directory `{}`: {err}", parent_dir.display())
        }
    }

    if let Err(err) = std::fs::write(&out_file, code)
    {
        panic!("cannot write code to file `{}`: {err}", out_file.display());
    }

    println!("cargo:rerun-if-changed={}", json_path.as_ref().display());
}
