#![forbid(unsafe_code)]
#![forbid(clippy::all)]

use std::{fs, path::Path, io::Result};

#[macro_use] extern crate pest_derive;

pub mod analyzer;
pub mod ast;
pub mod parser;

pub const DEFAULT_SCHEMA: &'static str = "public";

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<analyzer::SemanticSchemaBlock> {
  let raw_in = fs::read_to_string(path)?;

  let out_ast = parser::parse(&raw_in).unwrap_or_else(|e| panic!("{}", e));

  let sem_ast = out_ast.analyze().unwrap_or_else(|e| panic!("{}", e));

  Ok(sem_ast)
}