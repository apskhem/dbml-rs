use std::{ops::Range, str::FromStr, collections::HashMap};

use super::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableBlock {
  pub span_range: Range<usize>,
  pub cols: Vec<TableColumn>,
  pub ident: TableIdent,
  pub note: Option<String>,
  pub indexes: Option<indexes::IndexesBlock>,
  pub settings: Option<HashMap<String, Value>>,
  pub meta_indexer: TableIndexer
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ColumnType {
  pub span_range: Range<usize>,
  pub type_name: ColumnTypeName,
  pub args: Vec<Value>,
  pub arrays: Vec<Option<usize>>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableColumn {
  pub span_range: Range<usize>,
  pub name: String,
  pub r#type: ColumnType,
  pub settings: ColumnSettings,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableIndexer {
  pub pk_list: Vec<String>,
  pub unique_list: Vec<String>,
  pub indexed_list: Vec<String>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Integer(i32),
  Decimal(f32),
  Bool(bool),
  HexColor(String),
  Expr(String),
  Null
}

impl ToString for Value {
  fn to_string(&self) -> String {
    match self {
      Self::String(val) => format!("{}", val),
      Self::Integer(val) => format!("{}", val),
      Self::Decimal(val) => format!("{}", val),
      Self::Bool(val) => format!("{}", val),
      Self::HexColor(val) => format!("{}", val),
      Self::Expr(val) => format!("{}", val),
      Self::Null => format!("null")
    }
  }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum ColumnTypeName {
  /// The initial value (default)
  #[default] Undef,
  /// The type is waiting to be parsed and validated.
  Raw(String),
  Enum(String),
  Char,
  VarChar,
  SmallInt,
  Integer,
  BigInt,
  Real,
  DoublePrecision,
  Bool,
  ByteArray,
  Date,
  Text,
  Time,
  Timestamp,
  Timestampz,
  Uuid,
  Json,
  Decimal
}

impl FromStr for ColumnTypeName {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "char" => Ok(Self::Char),
      "varchar" => Ok(Self::VarChar),
      "smallint" => Ok(Self::SmallInt),
      "int2" => Ok(Self::SmallInt),
      "integer" => Ok(Self::Integer),
      "int" => Ok(Self::Integer),
      "int4" => Ok(Self::Integer),
      "bigint" => Ok(Self::BigInt),
      "int8" => Ok(Self::BigInt),
      "real" => Ok(Self::Real),
      "float4" => Ok(Self::Real),
      "float8" => Ok(Self::DoublePrecision),
      "bool" => Ok(Self::Bool),
      "boolean" => Ok(Self::Bool),
      "bytea" => Ok(Self::ByteArray),
      "date" => Ok(Self::Date),
      "text" => Ok(Self::Text),
      "time" => Ok(Self::Time),
      "timestamp" => Ok(Self::Timestamp),
      "timestampz" => Ok(Self::Timestampz),
      "uuid" => Ok(Self::Uuid),
      "json" => Ok(Self::Json),
      "decimal" => Ok(Self::Decimal),
      "numeric" => Ok(Self::Decimal),
      _ => Err(()),
    }
  }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ColumnSettings {
  pub span_range: Range<usize>,
  pub is_pk: bool,
  pub is_unique: bool,
  pub is_nullable: bool,
  pub is_incremental: bool,
  pub note: Option<String>,
  pub default: Option<Value>,
  pub refs: Vec<refs::RefInline>
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableIdent {
  pub span_range: Range<usize>,
  pub name: String,
  pub schema: Option<String>,
  pub alias: Option<String>
}