use std::str::FromStr;

use super::SpanRange;

/// A String literal.
#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
  pub span_range: SpanRange,
  pub value: Value,
  pub raw: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Ident {
  pub span_range: SpanRange,
  pub name: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct KeyValue {
  pub span_range: SpanRange,
  pub key: Ident,
  pub value: Option<Literal>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Nullable {
  NotNull,
  Null
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