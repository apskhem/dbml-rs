use std::str::FromStr;

use super::*;

/// A struct representing a block of table.
#[derive(Debug, Clone, Default)]
pub struct TableBlock {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// Columns or fields of the table.
  pub cols: Vec<TableColumn>,
  /// Identifier for the table.
  pub ident: TableIdent,
  /// The note for the table.
  pub note: Option<String>,
  /// The indexes block.
  pub indexes: Option<IndexesBlock>,
  /// The settings for the table.
  pub settings: Option<TableSettings>,
}

/// A struct representing settings of the table.
#[derive(Debug, Clone, Default)]
pub struct TableSettings {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// A vector of key-value pairs representing properties of the table.
  pub properties: Vec<KeyValue>,
}

/// A struct representing a single column or field of the table.
#[derive(Debug, Clone, Default)]
pub struct TableColumn {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The column name.
  pub name: Ident,
  /// The data type of the column.
  pub r#type: ColumnType,
  /// The settings of the column.
  pub settings: Option<ColumnSettings>,
}

/// A struct representing details of the table column.
#[derive(Debug, Clone, Default)]
pub struct ColumnType {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The raw type including type name, args, and arrays.
  pub raw: String,
  /// The parsed data type.
  pub type_name: ColumnTypeName,
  /// Type arguments.
  pub args: Vec<Value>,
  /// Type arrays.
  pub arrays: Vec<Option<u32>>,
}

/// Represents data types of the database.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
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

/// A struct representing settings of a column.
#[derive(Debug, Clone, Default)]
pub struct ColumnSettings {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// A vector of key-value pairs representing properties of the column.
  pub properties: Vec<KeyValue>,
  /// A boolean indicating if the column is a primary key.
  pub is_pk: bool,
  /// A boolean indicating if the column is unique.
  pub is_unique: bool,
  /// An enum indicating the nullable status of the column.
  pub is_nullable: Option<Nullable>,
  /// A boolean indicating if the column is incremental.
  pub is_incremental: bool,
  /// A note associated with the column.
  pub note: Option<String>,
  /// A default value for the column.
  pub default: Option<Value>,
  /// A vector of inline references associated with the column.
  pub refs: Vec<refs::RefInline>,
}

/// A struct representing a table identifier.
#[derive(Debug, Clone, Default)]
pub struct TableIdent {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The name of the table.
  pub name: Ident,
  /// The schema of the table, if specified.
  pub schema: Option<Ident>,
  /// The alias for the table.
  pub alias: Option<Ident>,
}
