use std::{str::FromStr, ops::Range};

use pest::Span;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum DatabaseType {
  #[default] Undef,
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
      _ => Err(format!("'{}' database is not supported!", s))
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ProjectBlock {
  pub span_range: Range<usize>,
  pub name: String,
  pub database_type: DatabaseType,
  pub note: Option<String>
}

impl From<Span<'_>> for ProjectBlock {
  fn from(span: Span<'_>) -> Self {
    Self {
      span_range: span.start()..span.end(),
      ..Default::default()
    }
  }
}