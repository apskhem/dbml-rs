use alloc::collections::BTreeSet;

use super::IndexesType;

/// A struct representing indexed meta data during parsing.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableIndexer {
  /// A list of primary column names (composition applicable).
  pub pk_list: Vec<String>,
  /// A list of column names with unique constraint  (composition applicable).
  pub unique_list: Vec<BTreeSet<String>>,
  /// A list of indexed column names (composition applicable).
  pub indexed_list: Vec<(Vec<String>, Option<IndexesType>)>,
}