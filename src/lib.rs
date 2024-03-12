#![forbid(unsafe_code)]
#![forbid(clippy::all)]

use std::result::Result;

use ast::SchemaBlock;
use pest::error::Error as ParseError;

extern crate derive_more;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate alloc;

pub(crate) mod analyzer;
pub mod ast;
pub(crate) mod parser;
#[cfg(not(feature = "utils"))]
pub(crate) mod utils;
#[cfg(feature = "utils")]
pub mod utils;

pub use analyzer::*;
pub use parser::{
  parse as parse_dbml_unchecked,
  Rule,
};

/// Default database schema if not specified in a DBML file.
pub const DEFAULT_SCHEMA: &str = "public";

/// Parses the given text input and performs a semantics check.
///
/// # Arguments
///
/// * `input` - A reference to a string containing the text content to be parsed.
///
/// # Returns
///
/// Returns a `Result` where:
/// - If parsing and semantic analysis are successful, it contains the resulting `analyzer::SemanticSchemaBlock`.
/// - If an error occurs during parsing or analysis, it contains a `ParseError` indicating the specific issue.
///
/// # Examples
///
/// ```rs
/// use your_crate_name::{parse_dbml, ParseError};
///
/// let content = "example content";
/// match parse_dbml(content) {
///     Ok(sem_ast) => {
///         // Successfully parsed and analyzed content.
///         // Work with the semantic analysis result (sem_ast) here.
///     }
///     Err(parse_error) => {
///         // Handle the parsing error.
///         eprintln!("Parsing error: {:?}", parse_error);
///     }
/// }
/// ```
pub fn parse_dbml(input: &str) -> Result<SchemaBlock, ParseError<Rule>> {
  let ast = parse_dbml_unchecked(input)?;

  analyze(&ast).map(|_| ast)
}
