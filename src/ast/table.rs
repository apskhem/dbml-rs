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
  /// The initial value (default).
  #[default] Undef,
  /// The type is waiting to be parsed and validated.
  Raw(String),
  Enum(String),
  Bit,
  Varbit,
  Char,
  VarChar,
  Box,
  Cidr,
  Circle,
  Inet,
  Line,
  LineSegment,
  MacAddr,
  MacAddr8,
  Money,
  Path,
  PGLongSequenceNumber,
  PGSnapshot,
  Point,
  Polygon,
  TSQuery,
  TSVector,
  SmallSerial,
  Serial,
  BigSerial,
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
  Timetz,
  Timestamp,
  Timestamptz,
  Uuid,
  Json,
  Jsonb,
  Decimal,
  Xml
}

impl FromStr for ColumnTypeName {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "bit" => Ok(Self::Bit),
      "varbit" => Ok(Self::Varbit),
      "bit varying" => Ok(Self::Varbit),
      "char" => Ok(Self::Char),
      "character" => Ok(Self::Char),
      "varchar" => Ok(Self::VarChar),
      "character varying" => Ok(Self::VarChar),
      "box" => Ok(Self::Box),
      "cidr" => Ok(Self::Cidr),
      "circle" => Ok(Self::Circle),
      "inet" => Ok(Self::Inet),
      "line" => Ok(Self::Line),
      "lseg" => Ok(Self::LineSegment),
      "macaddr" => Ok(Self::MacAddr),
      "macaddr8" => Ok(Self::MacAddr8),
      "money" => Ok(Self::Money),
      "path" => Ok(Self::Path),
      "pg_lsn" => Ok(Self::PGLongSequenceNumber),
      "pg_snapshot" => Ok(Self::PGSnapshot),
      "point" => Ok(Self::Point),
      "polygon" => Ok(Self::Polygon),
      "tsquery" => Ok(Self::TSQuery),
      "tsvector" => Ok(Self::TSVector),
      "smallserial" => Ok(Self::SmallSerial),
      "serial2" => Ok(Self::SmallSerial),
      "serial" => Ok(Self::Serial),
      "serial4" => Ok(Self::Serial),
      "bigserial" => Ok(Self::BigSerial),
      "serial8" => Ok(Self::BigSerial),
      "smallint" => Ok(Self::SmallInt),
      "int2" => Ok(Self::SmallInt),
      "integer" => Ok(Self::Integer),
      "int" => Ok(Self::Integer),
      "int4" => Ok(Self::Integer),
      "bigint" => Ok(Self::BigInt),
      "int8" => Ok(Self::BigInt),
      "real" => Ok(Self::Real),
      "float4" => Ok(Self::Real),
      "double precision" => Ok(Self::DoublePrecision),
      "float8" => Ok(Self::DoublePrecision),
      "bool" => Ok(Self::Bool),
      "boolean" => Ok(Self::Bool),
      "bytea" => Ok(Self::ByteArray),
      "date" => Ok(Self::Date),
      "text" => Ok(Self::Text),
      "time" => Ok(Self::Time),
      "time without time zone" => Ok(Self::Time),
      "timetz" => Ok(Self::Timetz),
      "time with time zone" => Ok(Self::Timetz),
      "timestamp" => Ok(Self::Timestamp),
      "timestamp without time zone" => Ok(Self::Timestamp),
      "timestamptz" => Ok(Self::Timestamptz),
      "timestamp with time zone" => Ok(Self::Timestamptz),
      "uuid" => Ok(Self::Uuid),
      "json" => Ok(Self::Json),
      "jsonb" => Ok(Self::Jsonb),
      "decimal" => Ok(Self::Decimal),
      "numeric" => Ok(Self::Decimal),
      "xml" => Ok(Self::Xml),
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