use std::str::FromStr;

use super::SpanRange;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesBlock {
  /// The range of the span.
  pub span_range: SpanRange,
  pub defs: Vec<IndexesDef>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesDef {
  /// The range of the span.
  pub span_range: SpanRange,
  pub cols: Vec<IndexesColumnType>,
  pub settings: Option<IndexesSettings>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesSettings {
  /// The range of the span.
  pub span_range: SpanRange,
  pub r#type: Option<IndexesType>,
  pub is_unique: bool,
  pub is_pk: bool,
  pub note: Option<String>,
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
