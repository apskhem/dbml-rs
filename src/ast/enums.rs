use super::SpanRange;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EnumBlock {
  /// The range of the span.
  pub span_range: SpanRange,
  pub ident: EnumIdent,
  pub values: Vec<EnumValue>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EnumValue {
  /// The range of the span.
  pub span_range: SpanRange,
  pub value: String,
  pub note: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EnumIdent {
  /// The range of the span.
  pub span_range: SpanRange,
  pub name: String,
  pub schema: Option<String>,
}
