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

impl DatabaseType {
  pub fn match_type(value: &str) -> Self {
    match value {
      "PostgreSQL" => Self::PostgreSQL,
      _ => unreachable!("'{:?}' database is not supported!", value)
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ProjectBlock {
  pub name: String,
  pub database_type: DatabaseType,
  pub note: Option<String>
}
