use std::ops::Range;

use super::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableBlock {
  pub span_range: Range<usize>,
  pub cols: Vec<TableColumn>,
  pub ident: TableIdent,
  pub note: Option<String>,
  pub indexes: Option<indexes::IndexesBlock>,
  pub indexer: TableIndexer
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
  Hex(String),
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
      Self::Hex(val) => format!("{}", val),
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

impl ColumnTypeName {
  pub fn match_type(value: &str) -> Result<Self, ()> {
    match value {
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

  pub fn to_col_type(&self, args: &Vec<Value>) -> Option<String> {
    let str_arg_vec: Vec<_> = args.iter().map(|arg| arg.to_string()).collect();
    let str_arg = if str_arg_vec.len() == 0 {
      format!("None")
    } else if str_arg_vec.len() == 1 {
      format!("Some({})", str_arg_vec.join(", "))
    } else {
      format!("Some(({}))", str_arg_vec.join(", "))
    };

    match self {
      Self::Char => Some(format!("Char({})", str_arg)),
      Self::VarChar => Some(format!("String({})", str_arg)),
      Self::SmallInt => Some(format!("SmallInteger")),
      Self::Integer => Some(format!("Integer")),
      Self::BigInt => Some(format!("BigInteger")),
      Self::Real => Some(format!("Float")),
      Self::DoublePrecision => Some(format!("Double")),
      Self::Bool => Some(format!("Boolean")),
      Self::ByteArray => Some(format!("Binary")),
      Self::Date => Some(format!("Date")),
      Self::Text => Some(format!("Text")),
      Self::Time => Some(format!("Time")),
      Self::Timestamp => Some(format!("DateTime")),
      Self::Timestampz => Some(format!("TimestampWithTimeZone")),
      Self::Uuid => Some(format!("Uuid")),
      Self::Json => Some(format!("Json")),
      Self::Decimal => Some(format!("Decimal({})", str_arg)),
      _ => None
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
  pub refs: Vec<refs::RefBlock>
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableIdent {
  pub span_range: Range<usize>,
  pub name: String,
  pub schema: Option<String>,
  pub alias: Option<String>
}