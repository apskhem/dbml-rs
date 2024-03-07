use super::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableGroupBlock {
  /// The range of the span.
  pub span_range: SpanRange,
  pub name: Ident,
  pub table_idents: Vec<TableGroupIdent>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableGroupIdent {
  /// The range of the span.
  pub span_range: SpanRange,
  pub raw: String,
  pub schema: Option<Ident>,
  pub ident_alias: Ident,
}
