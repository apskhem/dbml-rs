use crate::ast::*;

use super::indexer;

/// A validated reference block.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexedRefBlock {
  pub rel: refs::Relation,
  pub lhs: refs::RefIdent,
  pub rhs: refs::RefIdent,
  pub settings: Option<refs::RefSettings>,
}

impl IndexedRefBlock {
  pub fn from_inline(
    ref_blocks: Vec<refs::RefInline>,
    table_ident: table::TableIdent,
    col_name: String,
  ) -> Vec<Self> {
    ref_blocks
      .into_iter()
      .map(|ref_block| {
        let table_ident = table_ident.clone();
        let col_name = col_name.clone();

        let refs::RefInline { rel, rhs } = ref_block;

        let lhs = refs::RefIdent {
          schema: table_ident.schema,
          table: table_ident.name,
          compositions: vec![col_name],
        };

        Self {
          rel,
          lhs,
          rhs,
          settings: None,
        }
        .normalize()
      })
      .collect()
  }

  pub fn validate_ref_type(
    &self,
    tables: &Vec<table::TableBlock>,
    indexer: &indexer::Indexer,
  ) -> Result<(), String> {
    let lhs_ident = indexer.refer_ref_alias(&self.lhs);
    let rhs_ident = indexer.refer_ref_alias(&self.rhs);

    if lhs_ident.compositions.len() != rhs_ident.compositions.len() {
      return Err(format!(
        "relation composition must have number of fields equal in both side"
      ));
    }

    indexer.lookup_table_fields(&lhs_ident.schema, &lhs_ident.table, &lhs_ident.compositions)?;
    indexer.lookup_table_fields(&rhs_ident.schema, &rhs_ident.table, &rhs_ident.compositions)?;

    let lhs_table = tables
      .iter()
      .find(|table| table.ident.schema == lhs_ident.schema && table.ident.name == lhs_ident.table)
      .ok_or_else(|| format!("cannot find lhs table"))?;

    let rhs_table = tables
      .iter()
      .find(|table| table.ident.schema == rhs_ident.schema && table.ident.name == rhs_ident.table)
      .ok_or_else(|| format!("cannot find rhs table"))?;

    let field_pairs = lhs_ident
      .compositions
      .iter()
      .zip(rhs_ident.compositions.iter());

    for (l, r) in field_pairs.into_iter() {
      let l_field = lhs_table
        .cols
        .iter()
        .find(|col| &col.name == l)
        .ok_or_else(|| format!("cannot find l col"))?;
      let r_field = rhs_table
        .cols
        .iter()
        .find(|col| &col.name == r)
        .ok_or_else(|| format!("cannot find r col"))?;

      if l_field.r#type != r_field.r#type {
        return Err(format!("reference (composite) column type is mismatched"));
      }
    }

    Ok(())
  }

  pub fn eq(&self, other: &Self, indexer: &indexer::Indexer) -> bool {
    let self_ident = indexer.refer_ref_alias(&self.lhs);
    let other_ident = indexer.refer_ref_alias(&other.lhs);

    self_ident == other_ident
  }

  fn normalize(self) -> Self {
    // TODO:
    self
  }
}

impl From<refs::RefBlock> for IndexedRefBlock {
  fn from(ref_block: refs::RefBlock) -> Self {
    let refs::RefBlock {
      rel,
      lhs,
      rhs,
      settings,
    } = ref_block;

    Self {
      rel,
      lhs,
      rhs,
      settings,
    }
    .normalize()
  }
}
