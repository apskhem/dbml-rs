use std::str::FromStr;

use super::SpanRange;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesBlock {
  /// The range of the span.
  pub span_range: SpanRange,
  /// Defined items inside an indexes block.
  pub defs: Vec<IndexesDef>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesDef {
  /// The range of the span.
  pub span_range: SpanRange,
  /// Table column names for indexing which can be composite.
  pub cols: Vec<IndexesColumnType>,
  /// A Configuration for the specified column names.
  pub settings: Option<IndexesSettings>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesSettings {
  /// The range of the span.
  pub span_range: SpanRange,
  /// A Type of index (btree, gin, gist, hash depending on DB).
  pub r#type: Option<IndexesType>,
  /// A unique index.
  pub is_unique: bool,
  /// A primary index.
  pub is_pk: bool,
  /// A note.
  pub note: Option<String>,
  /// An index name.
  pub name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IndexesColumnType {
  String(String),
  Expr(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IndexesType {
  BTree,
  Gin,
  Gist,
  Hash,
}

impl FromStr for IndexesType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "btree" => Ok(Self::BTree),
      "gin" => Ok(Self::Gin),
      "gist" => Ok(Self::Gist),
      "hash" => Ok(Self::Hash),
      _ => Err(format!("'{}' type is not supported!", s)),
    }
  }
}
