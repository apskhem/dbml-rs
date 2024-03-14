use alloc::string::{
  String,
  ToString,
};
use core::str::FromStr;

use super::SpanRange;

/// Represents a string literal.
#[derive(Debug, Clone)]
pub struct Literal {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The value associated with the literal.
  pub raw: String,
  /// The value associated with the literal.
  pub value: Value,
}

/// Represents an identifier.
#[derive(Debug, Clone, Default)]
pub struct Ident {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The raw string value of the identifier.
  pub raw: String,
  /// The string representation of the identifier.
  pub to_string: String,
}

/// Represents an attribute with a key-value pair. It can have solely key without specified any value.
#[derive(Debug, Clone, Default)]
pub struct Attribute {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The key of the attribute.
  pub key: Ident,
  /// The value associated with the attribute, if any.
  pub value: Option<Literal>,
}

/// Represents a key-value property.
#[derive(Debug, Clone)]
pub struct Property {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// Identifier representing the key of the property.
  pub key: Ident,
  /// Literal value associated with the property.
  pub value: Literal,
}

/// Represents whether a value is explicitly specified as either null or not null.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Nullable {
  NotNull,
  Null,
}

/// Represents settings and arguments values.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Enum(String),
  String(String),
  Integer(i64),
  Decimal(f64),
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
      Self::Enum(v) => v.clone(),
      Self::String(v) => v.clone(),
      Self::Integer(v) => format!("{v}"),
      Self::Decimal(v) => format!("{v}"),
      Self::Bool(v) => format!("{v}"),
      Self::HexColor(v) => v.clone(),
      Self::Expr(v) => v.clone(),
      Self::Null => "null".to_string(),
    }
  }
}

/// Represents a note block.
#[derive(Debug, Clone)]
pub struct NoteBlock {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The literal value associated with the note block. It must be a string literal.
  pub value: Literal,
}
