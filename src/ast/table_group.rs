use super::*;

/// A table group allowing to group the related or associated tables together.
#[derive(Debug, Clone, Default)]
pub struct TableGroupBlock {
  /// The range of the span.
  pub span_range: SpanRange,
  /// A name of a table group
  pub ident: Ident,
  /// A list of tables inside a group.
  pub table_idents: Vec<TableGroupIdent>,
}

/// An associated table inside a table group.
#[derive(Debug, Clone, Default)]
pub struct TableGroupIdent {
  /// The range of the span.
  pub span_range: SpanRange,
  /// A Table schema.
  pub schema: Option<Ident>,
  /// A Table name or alias.
  pub ident_alias: Ident,
}
