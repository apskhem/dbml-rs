use super::SpanRange;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableGroupBlock {
  /// The range of the span.
  pub span_range: SpanRange,
  pub name: String,
  pub table_idents: Vec<TableGroupIdent>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableGroupIdent {
  /// The range of the span.
  pub span_range: SpanRange,
  pub raw: String,
  pub schema: Option<String>,
  pub ident_alias: String,
}
