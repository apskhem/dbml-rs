use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;

use super::*;

/// Represents an indexes block inside a table block.
/// Indexes allow users to quickly locate and access the data. Users can define single or multi-column indexes.
#[derive(Debug, Clone, Default)]
pub struct IndexesBlock {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// Defined items inside an indexes block.
  pub defs: Vec<IndexesDef>,
}

/// Represents an indexes definition or each item in an indexes block.
#[derive(Debug, Clone, Default)]
pub struct IndexesDef {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// Table column names for indexing which can be composite.
  pub cols: Vec<IndexesColumnType>,
  /// A Configuration for the specified column names.
  pub settings: Option<IndexesSettings>,
}

/// Represents settings of an indexes definition.
#[derive(Debug, Clone, Default)]
pub struct IndexesSettings {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// A vector of key and optional value pairs representing attributes of the indexes definition.
  pub attributes: Vec<Attribute>,
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

/// Represents the type of column for indexing.
#[derive(Debug, Clone)]
pub enum IndexesColumnType {
  /// Represents a column name with the given identifier.
  String(Ident),
  /// Represents an expression with the given literal expression.
  Expr(Literal),
}

/// Represents different types of indexes that can be used.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IndexesType {
  /// Represents a B-tree index.
  BTree,
  /// Represents a GIN (Generalized Inverted Index) index.
  Gin,
  /// Represents a GiST (Generalized Search Tree) index.
  Gist,
  /// Represents a hash index.
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
