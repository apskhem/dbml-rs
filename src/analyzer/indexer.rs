use alloc::collections::{
  BTreeMap,
  BTreeSet,
};
use alloc::string::{
  String,
  ToString,
};

use super::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct IndexedSchemaBlock {
  /// Indexed table names and associated columns
  table_map: BTreeMap<String, BTreeSet<String>>,
  /// Indexed enum names and associated variants
  enum_map: BTreeMap<String, BTreeSet<String>>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Indexer {
  /// Indexed table groups map.
  table_group_map: BTreeMap<String, BTreeSet<(String, String)>>,
  /// Indexed schema map.
  schema_map: BTreeMap<String, IndexedSchemaBlock>,
  /// Indexed alias map to the schema and table name.
  table_alias_map: BTreeMap<String, (String, String)>,
}

impl Indexer {
  /// Collects and validates table identifiers and their fields.
  ///
  /// # Errors
  ///
  /// - `DuplicateTableName`
  /// - `DuplicateColumnName`
  /// - `DuplicateAlias`
  pub(super) fn index_table(&mut self, tables: &Vec<&TableBlock>, input: &str) -> AnalyzerResult<()> {
    for table in tables {
      if let Some(settings) = &table.settings {
        check_attr_duplicate_keys(&settings.attributes, input)?;
      }

      let TableIdent {
        span_range,
        schema,
        name,
        alias,
      } = &table.ident;

      let schema = schema
        .as_ref()
        .map(|s| s.to_string.clone())
        .unwrap_or_else(|| DEFAULT_SCHEMA.to_string());

      if self.contains_table(&schema, &name.to_string) {
        throw_err(Err::DuplicateTableName, span_range, input)?;
      }

      let mut indexed_cols = BTreeSet::new();
      for col in table.cols.iter() {
        if let Some(settings) = &col.settings {
          check_attr_duplicate_keys(&settings.attributes, input)?;
        }

        if !indexed_cols.insert(col.name.to_string.clone()) {
          throw_err(Err::DuplicateColumnName, &col.span_range, input)?;
        }
      }

      self
        .schema_map
        .entry(schema.clone())
        .or_insert_with(IndexedSchemaBlock::default)
        .table_map
        .insert(name.to_string.clone(), indexed_cols);

      if let Some(alias) = alias {
        if let Some(_) = self
          .table_alias_map
          .insert(alias.to_string.clone(), (schema.clone(), name.to_string.clone()))
        {
          throw_err(Err::DuplicateAlias, &alias.span_range, input)?;
        };
      }
    }

    Ok(())
  }

  /// Collects and validates enum identifiers and their values.
  ///
  /// # Errors
  ///
  /// - `DuplicateEnumName`
  /// - `DuplicateEnumValue`
  pub(super) fn index_enums(&mut self, enums: &Vec<&EnumBlock>, input: &str) -> AnalyzerResult<()> {
    for r#enum in enums.iter() {
      let EnumIdent {
        span_range,
        schema,
        name,
        ..
      } = &r#enum.ident;

      let schema = schema
        .as_ref()
        .map(|s| s.to_string.clone())
        .unwrap_or_else(|| DEFAULT_SCHEMA.into());

      if self.contains_enum(&schema, &name.to_string) {
        throw_err(Err::DuplicateEnumName, &span_range, input)?;
      }

      let mut value_sets = BTreeSet::new();
      for value in r#enum.values.iter() {
        check_attr_duplicate_keys(&value.attributes, input)?;

        if !value_sets.insert(value.value.to_string.clone()) {
          throw_err(Err::DuplicateEnumValue, &value.span_range, input)?;
        }
      }

      self
        .schema_map
        .entry(schema)
        .or_insert_with(IndexedSchemaBlock::default)
        .enum_map
        .insert(name.to_string.clone(), value_sets);
    }

    Ok(())
  }

  /// Collects and validates table group identifiers and their items.
  ///
  /// # Errors
  ///
  /// - `DuplicateTableGroupName`
  /// - `TableNotFound`
  /// - `DuplicateTableGroupItem`
  pub(super) fn index_table_groups(&mut self, table_groups: &Vec<&TableGroupBlock>, input: &str) -> AnalyzerResult<()> {
    for table_group in table_groups {
      if self.table_group_map.get(&table_group.ident.to_string).is_some() {
        throw_err(Err::DuplicateTableGroupName, &table_group.ident.span_range, input)?;
      }

      let mut indexed_items = BTreeSet::new();
      for group_item in &table_group.items {
        let ident = match &group_item.schema {
          Some(item_schema) => (item_schema.to_string.clone(), group_item.ident_alias.to_string.clone()),
          None => {
            match self.resolve_alias(&group_item.ident_alias.to_string) {
              Some(ident) => ident.clone(),
              None => {
                let has_table = self
                  .schema_map
                  .get(DEFAULT_SCHEMA)
                  .iter()
                  .any(|item| item.table_map.contains_key(&group_item.ident_alias.to_string));

                if !has_table {
                  throw_err(Err::TableNotFound, &group_item.span_range, input)?;
                }

                (DEFAULT_SCHEMA.to_string(), group_item.ident_alias.to_string.clone())
              }
            }
          }
        };

        if !indexed_items.insert(ident) {
          throw_err(Err::DuplicateTableGroupItem, &group_item.span_range, input)?;
        }
      }

      self
        .table_group_map
        .insert(table_group.ident.to_string.clone(), indexed_items);
    }

    Ok(())
  }

  /// Checks if the specified table identifier exists.
  pub fn contains_table(&self, schema: &String, name: &String) -> bool {
    self
      .schema_map
      .get(schema)
      .iter()
      .any(|item| item.table_map.contains_key(name))
  }

  /// Checks if the specified enum identifier exists.
  pub fn contains_enum(&self, schema: &String, name: &String) -> bool {
    self
      .schema_map
      .get(schema)
      .iter()
      .any(|item| item.enum_map.contains_key(name))
  }

  /// Checks if the enum contains the specified values.
  pub fn lookup_enum_values(
    &self,
    schema: &Option<String>,
    enum_name: &String,
    values: &Vec<String>,
  ) -> (bool, (bool, Vec<bool>)) {
    let schema = schema.clone().unwrap_or_else(|| DEFAULT_SCHEMA.to_string());

    match self.schema_map.get(&schema) {
      Some(block) => {
        match block.enum_map.get(enum_name) {
          Some(value_set) => {
            let results = values.iter().map(|v| value_set.contains(v.as_str())).collect();

            (true, (true, results))
          }
          None => (true, (false, vec![false; values.len()])),
        }
      }
      None => (false, (false, vec![false; values.len()])),
    }
  }

  /// Checks if the table contains the specified fields.
  pub fn lookup_table_fields(
    &self,
    schema: &Option<Ident>,
    table: &Ident,
    fields: &Vec<Ident>,
    input: &str,
  ) -> AnalyzerResult<()> {
    let schema_span = schema.clone().map(|s| s.span_range).unwrap_or_default();
    let schema = schema
      .clone()
      .map(|s| s.to_string)
      .unwrap_or_else(|| DEFAULT_SCHEMA.into());

    match self.schema_map.get(&schema) {
      Some(block) => {
        match block.table_map.get(&table.to_string) {
          Some(col_set) => {
            let unlisted_fields: Vec<_> = fields
              .iter()
              .filter(|v| !col_set.contains(&v.to_string))
              .cloned()
              .collect();

            if let Some(first) = unlisted_fields.first() {
              throw_err(Err::ColumnNotFound, &first.span_range, input)?;
            }

            Ok(())
          }
          None => throw_err(Err::TableNotFound, &table.span_range, input),
        }
      }
      None => throw_err(Err::SchemaNotFound, &schema_span, input),
    }
  }

  /// Gets the schema (if has) and table name from the given alias.
  pub fn resolve_alias(&self, table_alias: &String) -> Option<&(String, String)> {
    self.table_alias_map.get(table_alias)
  }

  /// Gets a new ref block if it is pointing to an alias.
  pub fn resolve_ref_alias(&self, ident: &RefIdent) -> RefIdent {
    match self.resolve_alias(&ident.table.to_string) {
      Some((schema, table)) => {
        RefIdent {
          span_range: ident.span_range.clone(),
          schema: Some(Ident {
            span_range: 0..0, // FIXME:
            raw: schema.clone(),
            to_string: schema.clone(),
          }),
          table: Ident {
            span_range: ident.table.span_range.clone(),
            raw: table.clone(),
            to_string: table.clone(),
          },
          compositions: ident.compositions.clone(),
        }
      }
      None => ident.clone(),
    }
  }
}

/// A normalized reference block.
#[derive(Debug, Clone, Default)]
pub struct IndexedRef {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub rel: Relation,
  pub lhs: RefIdent,
  pub rhs: RefIdent,
  pub settings: Option<RefSettings>,
}

impl IndexedRef {
  pub fn from_inline(ref_blocks: Vec<RefInline>, table_ident: &TableIdent, col_name: &Ident) -> Vec<Self> {
    ref_blocks
      .into_iter()
      .map(|ref_block| {
        let RefInline { span_range, rel, rhs } = ref_block;

        let lhs = RefIdent {
          span_range: span_range.clone(),
          schema: table_ident.schema.clone(),
          table: table_ident.name.clone(),
          compositions: vec![col_name.clone()],
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

  pub fn validate_ref_type(&self, tables: &Vec<TableBlock>, indexer: &Indexer, input: &str) -> AnalyzerResult<()> {
    let lhs_ident = indexer.resolve_ref_alias(&self.lhs);
    let rhs_ident = indexer.resolve_ref_alias(&self.rhs);

    let composition_len = lhs_ident.compositions.len().max(rhs_ident.compositions.len());
    if lhs_ident.compositions.len() != rhs_ident.compositions.len() {
      throw_err(Err::MismatchedCompositeForeignKey, &self.span_range, input)?;
    }

    indexer.lookup_table_fields(&lhs_ident.schema, &lhs_ident.table, &lhs_ident.compositions, input)?;
    indexer.lookup_table_fields(&rhs_ident.schema, &rhs_ident.table, &rhs_ident.compositions, input)?;

    let find_ref_table = |ref_ident: &RefIdent| -> AnalyzerResult<&TableBlock> {
      tables
        .iter()
        .find(|table| {
          table.ident.schema.as_ref().map(|s| &s.to_string) == ref_ident.schema.as_ref().map(|s| &s.to_string)
            && table.ident.name.to_string == ref_ident.table.to_string
        })
        .map(Ok)
        .unwrap_or_else(|| throw_err(Err::TableNotFound, &ref_ident.span_range, input))
    };
    let find_ref_col = |col_ident: &Ident, table: &TableBlock| -> AnalyzerResult<TableColumn> {
      let col = table.cols.iter().find(|col| col.name.to_string == col_ident.to_string);

      match col {
        Some(col) => Ok(col.clone()),
        None => throw_err(Err::ColumnNotFound, &col_ident.span_range, input),
      }
    };
    let is_valid_composite = |compositions: &Vec<Ident>, table_indexes: &Option<IndexesBlock>| -> bool {
      table_indexes
        .as_ref()
        .map(|indexes| {
          indexes.defs.iter().any(|def_item| {
            compositions.len() == def_item.cols.len()
              && def_item.settings.as_ref().is_some_and(|s| s.is_pk || s.is_unique)
              && eq_elements(
                compositions.iter().map(|s| &s.to_string),
                def_item.cols.iter().filter_map(|s| {
                  match s {
                    IndexesColumnType::String(s) => Some(&s.to_string),
                    _ => None,
                  }
                }),
              )
          })
        })
        .unwrap_or_default()
    };

    let lhs_table = find_ref_table(&lhs_ident)?;
    let rhs_table = find_ref_table(&rhs_ident)?;

    let field_pairs = lhs_ident.compositions.iter().zip(rhs_ident.compositions.iter());

    for (l, r) in field_pairs {
      let l_col = find_ref_col(l, lhs_table)?;
      let r_col = find_ref_col(r, rhs_table)?;

      let l_type = &l_col.r#type;
      let r_type = &r_col.r#type;
      if l_type.type_name != r_type.type_name || l_type.args != r_type.args || l_type.arrays != r_type.arrays {
        let err = Err::MismatchedForeignKeyType {
          r_ident: r.to_string.clone(),
          l_ident: l.to_string.clone(),
          r_type: r_type.raw.clone(),
          l_type: l.raw.clone(),
        };

        throw_err(err, &self.span_range, input)?;
      }

      if composition_len == 1 {
        let err = match (
          &self.rel,
          l_col.settings.as_ref().is_some_and(|s| s.is_pk || s.is_unique),
          r_col.settings.as_ref().is_some_and(|s| s.is_pk || s.is_unique),
        ) {
          (Relation::One2One, false, false) => Some((InvalidForeignKeyErr::One2One, &self.span_range)),
          (Relation::Many2Many, false, false) => Some((InvalidForeignKeyErr::Many2Many, &self.span_range)),
          (Relation::One2Many, false, _) => {
            Some((InvalidForeignKeyErr::NitherUniqueKeyNorPrimaryKey, &self.lhs.span_range))
          }
          (Relation::Many2One, _, false) => {
            Some((InvalidForeignKeyErr::NitherUniqueKeyNorPrimaryKey, &self.rhs.span_range))
          }
          _ => None,
        };

        if let Some((err, span_range)) = err {
          throw_err(Err::InvalidForeignKey { err }, span_range, input)?;
        }
      }
    }

    match composition_len {
      2.. => {
        let is_valid_lhs = is_valid_composite(&self.lhs.compositions, &lhs_table.indexes);
        let is_valid_rhs = is_valid_composite(&self.rhs.compositions, &rhs_table.indexes);

        let err = match self.rel {
          Relation::One2One if !is_valid_lhs && !is_valid_rhs => {
            Some((InvalidForeignKeyErr::One2OneComposite, &self.span_range))
          }
          Relation::Many2Many if !is_valid_lhs || !is_valid_rhs => {
            Some((InvalidForeignKeyErr::Many2ManyComposite, &self.span_range))
          }
          Relation::One2Many if !is_valid_lhs => {
            Some((
              InvalidForeignKeyErr::NitherUniqueKeyNorPrimaryKeyComposite,
              &self.lhs.span_range,
            ))
          }
          Relation::Many2One if !is_valid_rhs => {
            Some((
              InvalidForeignKeyErr::NitherUniqueKeyNorPrimaryKeyComposite,
              &self.rhs.span_range,
            ))
          }
          _ => None,
        };

        if let Some((err, span_range)) = err {
          throw_err(Err::InvalidForeignKey { err }, span_range, input)?;
        }
      }
      _ => (),
    };

    Ok(())
  }

  /// Check if both relations point to the same column.
  /// Should use after calling `validate_ref_type`.
  pub fn occupy_same_column(&self, other: &Self, indexer: &Indexer) -> bool {
    let eq_ident = |lhs: &RefIdent, rhs: &RefIdent| -> bool {
      lhs.schema.as_ref().map(|s| &s.to_string) == rhs.schema.as_ref().map(|s| &s.to_string)
        && lhs.table.to_string == rhs.table.to_string
        && eq_elements(
          lhs.compositions.iter().map(|s| &s.to_string),
          rhs.compositions.iter().map(|s| &s.to_string),
        )
    };

    match (&self.rel, &other.rel) {
      (Relation::Many2Many, Relation::Many2Many) | (Relation::One2One, Relation::One2One) => {
        [&self.lhs, &self.rhs].iter().all(|self_side| {
          [&other.lhs, &other.rhs].iter().any(|other_side| {
            let self_ident = indexer.resolve_ref_alias(self_side);
            let other_ident = indexer.resolve_ref_alias(other_side);

            eq_ident(&self_ident, &other_ident)
          })
        })
      }
      _ => {
        // TODO: add support for one-to-one relation validation
        let self_occupied_ident = match &self.rel {
          Relation::Many2One => Some(&self.lhs),
          Relation::One2Many => Some(&self.rhs),
          _ => None,
        };
        let other_occupied_ident = match &other.rel {
          Relation::Many2One => Some(&other.lhs),
          Relation::One2Many => Some(&other.rhs),
          _ => None,
        };

        match (self_occupied_ident, other_occupied_ident) {
          (Some(self_ident), Some(other_ident)) => {
            let self_ident = indexer.resolve_ref_alias(self_ident);
            let other_ident = indexer.resolve_ref_alias(other_ident);

            eq_ident(&self_ident, &other_ident)
          }
          _ => false,
        }
      }
    }
  }
}

impl From<RefBlock> for IndexedRef {
  fn from(ref_block: RefBlock) -> Self {
    Self {
      span_range: ref_block.span_range,
      rel: ref_block.rel,
      lhs: ref_block.lhs,
      rhs: ref_block.rhs,
      settings: ref_block.settings,
    }
  }
}
