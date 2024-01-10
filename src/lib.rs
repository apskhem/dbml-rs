#![forbid(unsafe_code)]
#![forbid(clippy::all)]

use std::error::Error;
use std::fs;
use std::path::Path;
use std::result::Result;

use pest::error::Error as ParseError;

#[macro_use]
extern crate pest_derive;

pub mod analyzer;
pub mod ast;
pub mod parser;
pub mod utils;

pub const DEFAULT_SCHEMA: &str = "public";

/// Reads a file and parses the content of the file and performs semantics check.
pub fn parse_file<P: AsRef<Path>>(
  path: P,
) -> Result<analyzer::SemanticSchemaBlock, Box<dyn Error>> {
  let raw = fs::read_to_string(path)?;

  parse_content(&raw).or_else(|err| Err(err.into()))
}

/// Parses the text content and performs semantics check.
pub fn parse_content(
  content: &str,
) -> Result<analyzer::SemanticSchemaBlock, ParseError<parser::Rule>> {
  let out_ast = parser::parse(content)?;

  let sem_ast = out_ast.analyze()?;

  Ok(sem_ast)
}
