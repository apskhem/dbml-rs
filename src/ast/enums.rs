use super::SpanRange;

#[derive(Debug, Clone, Default)]
pub struct EnumBlock {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub ident: EnumIdent,
  pub values: Vec<EnumValue>,
}

#[derive(Debug, Clone, Default)]
pub struct EnumValue {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub value: String,
  pub note: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EnumIdent {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub name: String,
  pub schema: Option<String>,
}
