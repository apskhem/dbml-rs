use crate::analyzer::*;
use crate::ast::*;

/// Gets all relations associated with a table, including references to other tables, references from other tables, and self-references.
///
/// # Arguments
///
/// * `table_ident` - A reference to the table identifier for which relations are to be fetched.
/// * `analyzed_indexer` - A reference to the analyzed indexer containing information about indexed references.
///
/// # Returns
///
/// A `TableRef` struct containing three vectors of `block::IndexedRefBlock` instances.
///
/// # Examples
///
/// ```rs
/// use dbml_rs::{get_table_refs, TableIdent, AnalyzedIndexer};
///
/// let table_ident = TableIdent {
///     span_range: 0..0,
///     name: "users".to_string(),
///     schema: Some("public".to_string()),
///     alias: None,
/// };
///
/// let analyzed_indexer = AnalyzedIndexer::default(); // Assuming some analyzed indexer is available
///
/// let table_refs = get_table_refs(&table_ident, &analyzed_indexer);
/// // Now `table_refs` contains all relations associated with the specified table.
/// ```
pub fn get_table_refs(table_ident: &TableIdent, analyzed_indexer: &AnalyzedIndexer) -> TableRef {
  let mut ref_to_blocks = vec![];
  let mut ref_by_blocks = vec![];
  let mut ref_self_blocks = vec![];

  let eq = |table_ident: &TableIdent, ref_ident: &RefIdent| {
    table_ident.schema.clone().map(|s| s.to_string) == ref_ident.schema.clone().map(|s| s.to_string)
      && table_ident.name.to_string == ref_ident.table.to_string
  };

  for ref_block in analyzed_indexer.indexed_refs.iter() {
    let lhs_ident = analyzed_indexer.indexer.resolve_ref_alias(&ref_block.lhs);
    let rhs_ident = analyzed_indexer.indexer.resolve_ref_alias(&ref_block.rhs);

    if eq(table_ident, &lhs_ident) && eq(table_ident, &rhs_ident) {
      ref_self_blocks.push(ref_block.clone())
    } else if eq(table_ident, &lhs_ident) {
      ref_to_blocks.push(ref_block.clone())
    } else if eq(table_ident, &rhs_ident) {
      ref_by_blocks.push(ref_block.clone())
    }
  }

  TableRef {
    ref_to: ref_to_blocks,
    ref_by: ref_by_blocks,
    ref_self: ref_self_blocks,
  }
}
