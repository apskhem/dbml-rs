#![forbid(unsafe_code)]
#![forbid(clippy::all)]

use std::{fs, path::Path, io::Result};

#[macro_use] extern crate pest_derive;

mod analyzer;
mod ast;
mod parser;
mod utils;

pub const DEFAULT_SCHEMA: &'static str = "public";

/// Read a file and parse the content of the file.
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<analyzer::SemanticSchemaBlock> {
  let raw = fs::read_to_string(path)?;

  let out_ast = parser::parse(&raw).unwrap_or_else(|e| panic!("{}", e));

  let sem_ast = out_ast.analyze().unwrap_or_else(|e| panic!("{}", e));

  Ok(sem_ast)
}