use alloc::vec::Vec;

use super::*;

/// Represents a table group allowing to group the related or associated tables together.
#[derive(Debug, Clone, Default)]
pub struct TableGroupBlock {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The name of a table group
  pub ident: Ident,
  /// The note block associated with the table group block.
  pub note: Option<NoteBlock>,
  /// The list of table identifiers inside the group.
  pub items: Vec<TableGroupItem>,

  /// The settings for the table group.
  pub settings: Option<TableGroupSettings>,
}

/// Represents settings of the table group.
#[derive(Debug, Clone, Default)]
pub struct TableGroupSettings {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// A vector of key and optional value pairs representing attributes of the table.
  pub attributes: Vec<Attribute>,
}

/// Represents an associated table identifier listed inside a table group.
#[derive(Debug, Clone, Default)]
pub struct TableGroupItem {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The table schema.
  pub schema: Option<Ident>,
  /// The table name or alias.
  pub ident_alias: Ident,
}
