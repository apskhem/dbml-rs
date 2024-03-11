use std::{collections::{
  HashMap,
  HashSet,
}, ops::Deref};

use super::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct IndexedSchemaBlock {
  /// Indexed table names and associated columns
  table_map: HashMap<String, HashSet<String>>,
  /// Indexed enum names and associated variants
  enum_map: HashMap<String, HashSet<String>>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Indexer {
  /// Indexed table groups map.
  table_group_map: HashMap<String, HashSet<(Option<String>, String)>>,
  /// Indexed schema map.
  schema_map: HashMap<String, IndexedSchemaBlock>,
  /// Indexed alias map to the schema (optional) and table name.
  table_alias_map: HashMap<String, (Option<String>, String)>,
}

impl Indexer {
  /// Collects and validates table identifiers and their fields.
  /// 
  /// # Errors
  /// 
  /// - `DuplicatedTableName`
  /// - `DuplicatedColumnName`
  /// - `DuplicatedAlias`
  pub(super) fn index_table(&mut self, tables: &Vec<&TableBlock>, input: &str) -> AnalyzerResult<()> {
    for table in tables {
      let TableIdent {
        span_range,
        schema,
        name,
        alias,
      } = &table.ident;

      let schema = schema.as_ref().map(|s| s.to_string.clone()).unwrap_or_else(|| DEFAULT_SCHEMA.to_owned());

      let has_dup_ident = self.schema_map
        .get(&schema)
        .iter()
        .any(|item| item.table_map.contains_key(&name.to_string));

      if has_dup_ident {
        throw_err(Err::DuplicatedTableName, &span_range, input)?;
      }

      let mut indexed_cols = HashSet::new();
      for col in table.cols.iter() {
        match indexed_cols.get(&col.name.to_string) {
          Some(col_name) => throw_err(Err::DuplicatedColumnName, &col.span_range, input)?,
          None => indexed_cols.insert(col.name.to_string.clone())
        };
      }

      match self.schema_map.get_mut(&schema) {
        Some(index_block) => {
          index_block.table_map.insert(name.to_string.clone(), indexed_cols);

          if let Some(alias) = alias {
            match self.table_alias_map.get(&alias.to_string) {
              Some(dup_alias) => {
                throw_err(Err::DuplicatedAlias, &alias.span_range, input)?;
              },
              None => {
                self
                  .table_alias_map
                  .insert(alias.to_string.clone(), (Some(schema.clone()), name.to_string.clone()));
              }
            };
          }
        }
        None => {
          let mut index_block = IndexedSchemaBlock::default();

          index_block.table_map.insert(name.to_string.clone(), indexed_cols);

          if let Some(alias) = alias {
            self
              .table_alias_map
              .insert(alias.to_string.clone(), (Some(schema.clone()), name.to_string.clone()));
          }

          self.schema_map.insert(schema, index_block);
        }
      }
    }

    Ok(())
  }

  /// Collects and validates enum identifiers and their values.
  pub(super) fn index_enums(&mut self, enums: &Vec<&EnumBlock>, input: &str) -> AnalyzerResult<()> {
    for r#enum in enums.iter() {
      let EnumIdent { schema, name, .. } = r#enum.ident.clone();

      let schema_name = schema.clone().map(|s| s.to_string.clone()).unwrap_or_else(|| DEFAULT_SCHEMA.into());
      let mut value_sets = HashSet::new();

      for value in r#enum.values.iter() {
        match value_sets.get(&value.value.to_string) {
          Some(dup_col_name) => throw_err(Err::DuplicatedEnumValue, &value.span_range, input)?,
          None => value_sets.insert(value.value.to_string.clone())
        };
      }

      match self.schema_map.get_mut(&schema_name) {
        Some(index_block) => {
          index_block.enum_map.insert(name.to_string.clone(), value_sets);
        }
        None => {
          let mut index_block = IndexedSchemaBlock::default();

          index_block.enum_map.insert(name.to_string.clone(), value_sets);

          self.schema_map.insert(schema_name, index_block);
        }
      }
    }

    Ok(())
  }

  /// Collects and validates table group identifiers and their items.
  /// 
  /// # Errors
  /// 
  /// - `DuplicatedTableGroupName`
  /// - `TableNotFound`
  /// - `DuplicatedTableGroupItem`
  pub(super) fn index_table_groups(
    &mut self,
    table_groups: &Vec<&TableGroupBlock>,
    input: &str,
  ) -> AnalyzerResult<()> {
    for table_group in table_groups {
      if self.table_group_map.get(&table_group.ident.to_string).is_some() {
        throw_err(Err::DuplicatedTableGroupName, &table_group.ident.span_range, input)?;
      }

      let mut indexed_items = HashSet::new();
      for group_item in &table_group.items {
        let ident = match &group_item.schema {
          Some(item_schema) => {
            (Some(item_schema.to_string.clone()), group_item.ident_alias.to_string.clone())
          }
          None => {
            match self.resolve_alias(&group_item.ident_alias.to_string) {
              Some(ident) => {
                ident.clone()
              },
              None => {
                let has_table = self.schema_map
                  .get(DEFAULT_SCHEMA)
                  .iter()
                  .any(|item| item.table_map.contains_key(&group_item.ident_alias.to_string));

                if !has_table {
                  throw_err(Err::TableNotFound, &group_item.span_range, input)?;
                }

                (Some(DEFAULT_SCHEMA.to_string()), group_item.ident_alias.to_string.clone())
              }
            }
          }
        };

        match indexed_items.get(&ident) {
          Some(_) => throw_err(Err::DuplicatedTableGroupItem, &group_item.span_range, input)?,
          None => indexed_items.insert(ident),
        };
      }

      self
        .table_group_map
        .insert(table_group.ident.to_string.clone(), indexed_items);
    }

    Ok(())
  }

  pub fn lookup_enum_values(
    &self,
    schema: &Option<String>,
    enum_name: &String,
    values: &Vec<String>,
  ) -> AnalyzerResult<()> {
    let schema = schema.clone().unwrap_or_else(|| DEFAULT_SCHEMA.into());

    match self.schema_map.get(&schema) {
      Some(block) => {
        match block.enum_map.get(enum_name) {
          Some(value_set) => {
            for v in values.iter() {
              if !value_set.contains(v) {
                panic!("not found '{}' value in enum '{}'", v, enum_name);
              }
            }
  
            Ok(())
          },
          None => {
            panic!("enum_not_found");
          }
        }
      }
      None => {
        panic!("schema_not_found");
      }
    }
  }

  pub fn lookup_table_fields(
    &self,
    schema: &Option<Ident>,
    table: &Ident,
    fields: &Vec<Ident>,
  ) -> AnalyzerResult<()> {
    let schema = schema.clone().map(|s| s.to_string).unwrap_or_else(|| DEFAULT_SCHEMA.into());

    if let Some(block) = self.schema_map.get(&schema) {
      if let Some(col_set) = block.table_map.get(&table.to_string) {
        let unlisted_fields: Vec<_> = fields
          .iter()
          .filter(|v| !col_set.contains(v.to_string.deref()))
          .cloned()
          .collect();

        match unlisted_fields.is_empty() {
          true => return Ok(()),
          false => {
            panic!(
              "not found '{}' column in table '{}'",
              unlisted_fields.iter().map(|s| s.to_string.clone()).collect::<Vec<_>>().join(", "),
              table.to_string
            );
          }
        }
      }

      panic!("table_not_found");
    }

    panic!("table_not_found");
  }

  /// Gets the schema (if has) and table name from the given alias.
  pub fn resolve_alias(&self, table_alias: &String) -> Option<&(Option<String>, String)> {
    self.table_alias_map.get(table_alias)
  }

  pub fn resolve_ref_alias(&self, ident: &RefIdent) -> RefIdent {
    match self.resolve_alias(&ident.table.to_string) {
      Some((schema, table)) => RefIdent {
        span_range: ident.span_range.clone(),
        schema: schema.clone().map(|s| Ident {
          span_range: 0..0,
          raw: s.clone(),
          to_string: s
        }),
        table: Ident {
          span_range: ident.table.span_range.clone(),
          raw: table.clone(),
          to_string: table.clone()
        },
        compositions: ident.compositions.clone(),
      },
      None => ident.clone(),
    }
  }
}
