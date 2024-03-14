use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;

use super::*;

/// Represents different types of databases.
#[derive(Debug, Clone, Default)]
pub enum DatabaseType {
  #[default]
  Undef,
  PostgreSQL,
  Oracle,
  MySQL,
  MongoDB,
  MSSQL,
  SQLite,
  MariaDB,
}

impl FromStr for DatabaseType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "PostgreSQL" => Ok(Self::PostgreSQL),
      _ => Err(format!("'{}' database is not supported", s)),
    }
  }
}

/// Represents a project block for grouping various tables.
#[derive(Debug, Clone, Default)]
pub struct ProjectBlock {
  /// Range of the span in the source text.
  pub span_range: SpanRange,
  /// Properties associated with the project block.
  pub properties: Vec<Property>,
  /// An identifier of the project block.
  pub ident: Ident,
  /// The database type associated with the project block.
  pub database_type: DatabaseType,
  /// The note block associated with the project block.
  pub note: Option<NoteBlock>,
}
