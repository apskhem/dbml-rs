use super::{table::Value, SpanRange};

/// A String literal.
#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
  pub span_range: SpanRange,
  pub value: Value,
  pub raw: String,
}
