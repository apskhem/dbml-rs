use super::*;

/// An entire structure of a parsed DBML file.
#[derive(Debug, Clone, Default)]
pub struct SchemaBlock<'a> {
  pub span_range: SpanRange,
  /// Input source content.
  pub input: &'a str,
  /// Overall description of the project. This is optional. The file must contain one or zero 'Project' block.
  pub project: Option<project::ProjectBlock>,
  /// Table block.
  pub tables: Vec<table::TableBlock>,
  /// TableGroup block.
  pub table_groups: Vec<table_group::TableGroupBlock>,
  /// Ref block.
  pub refs: Vec<refs::RefBlock>,
  /// Enums block.
  pub enums: Vec<enums::EnumBlock>,
}
