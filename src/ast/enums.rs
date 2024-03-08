use super::*;

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
  pub value: Ident,
  pub note: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EnumIdent {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub name: Ident,
  pub schema: Option<Ident>,
}
