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
  pub indexes: Option<IndexesBlock>,
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
  pub settings: Option<ColumnSettings>,
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
    let v = match s {
      "bit" => Self::Bit,
      "varbit" | "bit varying" => Self::Varbit,
      "char" | "character" => Self::Char,
      "varchar" | "character varying" => Self::VarChar,
      "box" => Self::Box,
      "cidr" => Self::Cidr,
      "circle" => Self::Circle,
      "inet" => Self::Inet,
      "line" => Self::Line,
      "lseg" => Self::LineSegment,
      "macaddr" => Self::MacAddr,
      "macaddr8" => Self::MacAddr8,
      "money" => Self::Money,
      "path" => Self::Path,
      "pg_lsn" => Self::PGLongSequenceNumber,
      "pg_snapshot" => Self::PGSnapshot,
      "point" => Self::Point,
      "polygon" => Self::Polygon,
      "tsquery" => Self::TSQuery,
      "tsvector" => Self::TSVector,
      "smallserial" | "serial2" => Self::SmallSerial,
      "serial" | "serial4" => Self::Serial,
      "bigserial" | "serial8" => Self::BigSerial,
      "smallint" | "int2" => Self::SmallInt,
      "integer" | "int" | "int4" => Self::Integer,
      "bigint" | "int8" => Self::BigInt,
      "real" | "float4" => Self::Real,
      "double precision" | "float8" => Self::DoublePrecision,
      "bool" | "boolean" => Self::Bool,
      "bytea" => Self::ByteArray,
      "date" => Self::Date,
      "text" => Self::Text,
      "time" | "time without time zone" => Self::Time,
      "timetz" | "time with time zone" => Self::Timetz,
      "timestamp" | "timestamp without time zone" => Self::Timestamp,
      "timestamptz" | "timestamp with time zone" => Self::Timestamptz,
      "uuid" => Self::Uuid,
      "json" => Self::Json,
      "jsonb" => Self::Jsonb,
      "decimal" | "numeric" => Self::Decimal,
      "xml" => Self::Xml,
      _ => return Err(()),
    };

    Ok(v)
  }
}

/// Column settings.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct ColumnSettings {
  pub span_range: SpanRange,
  pub properties: Vec<KeyValue>,
  pub is_pk: bool,
  pub is_unique: bool,
  pub is_nullable: Option<Nullable>,
  pub is_incremental: bool,
  pub note: Option<String>,
  pub default: Option<Value>,
  pub refs: Vec<refs::RefInline>,
}

/// A table identifier.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableIdent {
  pub span_range: SpanRange,
  pub name: String,
  pub schema: Option<String>,
  pub alias: Option<String>,
}
