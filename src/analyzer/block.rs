use crate::ast::*;

use super::*;

/// A normalized reference block.
#[derive(Debug, Clone, Default)]
pub struct IndexedRefBlock {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub rel: Relation,
  pub lhs: RefIdent,
  pub rhs: RefIdent,
  pub settings: Option<RefSettings>,
}

impl IndexedRefBlock {
  pub fn from_inline(
    ref_blocks: Vec<RefInline>,
    table_ident: TableIdent,
    col_name: Ident,
  ) -> Vec<Self> {
    ref_blocks
      .into_iter()
      .map(|ref_block| {
        let table_ident = table_ident.clone();
        let col_name = col_name.clone();

        let RefInline { span_range, rel, rhs } = ref_block;

        let lhs = RefIdent {
          span_range: span_range.clone(),
          schema: table_ident.schema.clone(),
          table: table_ident.name.clone(),
          compositions: vec![col_name],
        };

        Self {
          span_range,
          rel,
          lhs,
          rhs,
          settings: None,
        }
      })
      .collect()
  }

  pub fn validate_ref_type(
    &self,
    tables: &Vec<TableBlock>,
    indexer: &indexer::Indexer,
    input: &str,
  ) -> AnalyzerResult<()> {
    let lhs_ident = indexer.resolve_ref_alias(&self.lhs);
    let rhs_ident = indexer.resolve_ref_alias(&self.rhs);

    if lhs_ident.compositions.len() != rhs_ident.compositions.len() {
      panic!("relation composition must have number of fields equal in both side");
    }

    indexer.lookup_table_fields(&lhs_ident.schema, &lhs_ident.table, &lhs_ident.compositions)?;
    indexer.lookup_table_fields(&rhs_ident.schema, &rhs_ident.table, &rhs_ident.compositions)?;

    let lhs_table = tables
      .iter()
      .find(|table| {
        table.ident.schema.clone().map(|s| s.to_string) == lhs_ident.schema.clone().map(|s| s.to_string)
        && table.ident.name.to_string == lhs_ident.table.to_string
      })
      .ok_or_else(|| panic!("cannot find lhs table"))?;

    let rhs_table = tables
      .iter()
      .find(|table| {
        table.ident.schema.clone().map(|s| s.to_string) == rhs_ident.schema.clone().map(|s| s.to_string)
        && table.ident.name.to_string == rhs_ident.table.to_string
      })
      .ok_or_else(|| panic!("cannot find rhs table"))?;

    let field_pairs = lhs_ident
      .compositions
      .iter()
      .zip(rhs_ident.compositions.iter());

    for (l, r) in field_pairs.into_iter() {
      let l_field = lhs_table
        .cols
        .iter()
        .find(|col| col.name.to_string == l.to_string)
        .ok_or_else(|| panic!("cannot find l col"))?;
      let r_field = rhs_table
        .cols
        .iter()
        .find(|col| col.name.to_string == r.to_string)
        .ok_or_else(|| panic!("cannot find r col"))?;

      let l_type = &l_field.r#type;
      let r_type = &r_field.r#type;
      if l_type.type_name != r_type.type_name || l_type.args != r_type.args || l_type.arrays != r_type.arrays {
        panic!("reference (composite) column type is mismatched");
      }
    }

    Ok(())
  }

  pub fn eq(&self, other: &Self, indexer: &indexer::Indexer) -> bool {
    let self_ident = indexer.resolve_ref_alias(&self.lhs);
    let other_ident = indexer.resolve_ref_alias(&other.lhs);

    self_ident.compositions.iter().map(|s| s.to_string.clone()).collect::<Vec<_>>() == other_ident.compositions.iter().map(|s| s.to_string.clone()).collect::<Vec<_>>()
    && self_ident.schema.map(|s| s.to_string) == other_ident.schema.map(|s| s.to_string)
    && self_ident.table.to_string == other_ident.table.to_string
  }
}

impl From<RefBlock> for IndexedRefBlock {
  fn from(ref_block: RefBlock) -> Self {
    let RefBlock {
      rel,
      lhs,
      rhs,
      settings,
      span_range,
    } = ref_block;

    Self {
      span_range,
      rel,
      lhs,
      rhs,
      settings,
    }
  }
}
