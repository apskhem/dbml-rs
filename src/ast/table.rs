use std::str::FromStr;

use super::*;

/// A single declared block of table.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableBlock {
  /// The range of the span.
  pub span_range: SpanRange,
  /// Columns or fields of the table.
  pub cols: Vec<TableColumn>,
  /// Identifier for the table.
  pub ident: TableIdent,
  /// A note for the table.
  pub note: Option<String>,
  /// A indexes block.
  pub indexes: Option<indexes::IndexesBlock>,
  /// A settings for the table.
  pub settings: Option<TableSettings>,
  /// Meta indexer for the table.
  pub meta_indexer: TableIndexer,
}

/// A struct representing settings of the table.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableSettings {
  /// The range of the span.
  pub span_range: SpanRange,
  /// Settings values.
  pub values: Vec<(String, Value)>,
}

/// A single declared column of the table.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableColumn {
  /// The range of the span.
  pub span_range: SpanRange,
  /// A table name.
  pub name: String,
  /// A data type of the column.
  pub r#type: ColumnType,
  /// A settings for the column.
  pub settings: ColumnSettings,
}

/// A struct representing details of the table column.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct ColumnType {
  /// The range of the span.
  pub span_range: SpanRange,
  /// A parsed data type.
  pub type_name: ColumnTypeName,
  /// Type arguments.
  pub args: Vec<Value>,
  /// Type arrays.
  pub arrays: Vec<Option<u32>>,
}

/// A struct representing indexed meta data during parsing.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableIndexer {
  /// A list of primary column names
  pub pk_list: Vec<String>,
  /// A list of column names with unique constraint.
  pub unique_list: Vec<String>,
  /// A list of indexed column names
  pub indexed_list: Vec<String>,
}

/// Represents settings and arguments values.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Integer(i32),
  Decimal(f32),
  Bool(bool),
  HexColor(String),
  Expr(String),
  Null,
}

impl FromStr for Value {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "true" => Ok(Value::Bool(true)),
      "false" => Ok(Value::Bool(false)),
      "null" => Ok(Value::Null),
      _ => Err(()),
    }
  }
}

impl ToString for Value {
  fn to_string(&self) -> String {
    match self {
      Self::String(v) => format!("{v}"),
      Self::Integer(v) => format!("{v}"),
      Self::Decimal(v) => format!("{v}"),
      Self::Bool(v) => format!("{v}"),
      Self::HexColor(v) => format!("{v}"),
      Self::Expr(v) => format!("{v}"),
      Self::Null => format!("null"),
    }
  }
}

/// Represents data types of the database.
#[derive(Debug, PartialEq, Clone, Default)]
pub enum ColumnTypeName {
  /// An initial value (default).
  /// This should not present as a final parsing result.
  #[default]
  Undef,
  /// A type waiting to be parsed and validated.
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
  Xml,
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

/// Column settings.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct ColumnSettings {
  pub span_range: SpanRange,
  pub is_pk: bool,
  pub is_unique: bool,
  pub is_nullable: bool,
  pub is_incremental: bool,
  pub note: Option<String>,
  pub default: Option<Value>,
  pub refs: Vec<refs::RefInline>,
}

impl From<SpanRange> for ColumnSettings {
  fn from(value: SpanRange) -> Self {
    Self { span_range: value, ..Default::default() }
  }
}

/// A table identifier.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableIdent {
  pub span_range: SpanRange,
  pub name: String,
  pub schema: Option<String>,
  pub alias: Option<String>,
}

impl From<SpanRange> for TableIdent {
  fn from(value: SpanRange) -> Self {
    Self { span_range: value, ..Default::default() }
  }
}
