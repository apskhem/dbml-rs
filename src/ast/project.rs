use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;

use super::*;

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

#[derive(Debug, Clone, Default)]
pub struct ProjectBlock {
  pub span_range: SpanRange,
  pub properties: Vec<Property>,
  pub ident: Ident,
  pub database_type: DatabaseType,
  pub note: Option<NoteBlock>,
}
