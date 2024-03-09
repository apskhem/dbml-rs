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
  pub fn index_table(&mut self, tables: &Vec<&TableBlock>, input: &str) -> AnalyzerResult<()> {
    for table in tables {
      let TableIdent {
        span_range,
        schema,
        name,
        alias,
      } = &table.ident;

      let mut col_sets = HashSet::new();
      for col in table.cols.iter() {
        match col_sets.get(&col.name.to_string) {
          Some(col_name) => throw_err(Err::DuplicatedColumnName, &col.span_range, input)?,
          None => col_sets.insert(col.name.to_string.clone())
        };
      }

      let schema_name = schema.clone().map(|s| s.to_string).unwrap_or_else(|| DEFAULT_SCHEMA.into());
      match self.schema_map.get_mut(&schema_name) {
        Some(index_block) => {
          index_block.table_map.insert(name.to_string.clone(), col_sets);

          if let Some(alias) = alias {
            match self.table_alias_map.get(&alias.to_string) {
              Some(dup_alias) => panic!("alias_name_dup"),
              None => {
                self
                  .table_alias_map
                  .insert(alias.to_string.clone(), (schema.clone().map(|s| s.to_string), name.to_string.clone()))
              }
            };
          }
        }
        None => {
          let mut index_block = IndexedSchemaBlock::default();

          index_block.table_map.insert(name.to_string.clone(), col_sets);

          if let Some(alias) = alias {
            self
              .table_alias_map
              .insert(alias.to_string.clone(), (schema.clone().map(|s| s.to_string), name.to_string.clone()));
          }

          self.schema_map.insert(schema_name, index_block);
        }
      }
    }

    Ok(())
  }

  pub fn index_enums(&mut self, enums: &Vec<&EnumBlock>) -> AnalyzerResult<()> {
    for r#enum in enums.iter() {
      let EnumIdent { schema, name, .. } = r#enum.ident.clone();

      let schema_name = schema.clone().map(|s| s.to_string.clone()).unwrap_or_else(|| DEFAULT_SCHEMA.into());
      let mut value_sets = HashSet::new();

      for value in r#enum.values.iter() {
        match value_sets.get(&value.value.to_string) {
          Some(dup_col_name) => panic!("val_dup"),
          None => value_sets.insert(value.value.to_string.clone())
        };
      }

      if let Some(index_block) = self.schema_map.get_mut(&schema_name) {
        index_block.enum_map.insert(name.to_string.clone(), value_sets);
      } else {
        let mut index_block = IndexedSchemaBlock::default();

        index_block.enum_map.insert(name.to_string.clone(), value_sets);

        self.schema_map.insert(schema_name, index_block);
      }
    }

    Ok(())
  }

  pub fn index_table_groups(
    &mut self,
    table_groups: &Vec<&TableGroupBlock>,
    input: &str,
  ) -> AnalyzerResult<()> {
    for group_each in table_groups.into_iter() {
      for table in group_each.items.iter() {
        let ident_alias = table.ident_alias.clone();

        let ident = if let Some(ident) = self.table_alias_map.get(&ident_alias.to_string) {
          if table.schema.is_some() {
            panic!("alias_must_not_followed_by_schema")
          }

          ident.1.clone()
        } else {
          ident_alias.to_string
        };

        self.lookup_table_fields(&table.schema, &Ident { span_range: 0..0, raw: String::new(), to_string: ident }, &vec![])?;
      }

      let mut table_sets = HashSet::new();

      for table_ident in group_each.items.iter() {
        if let Some(ident) = self.resolve_alias(&table_ident.ident_alias.to_string) {
          if let Some(_) = table_sets.get(ident) {
            panic!("table_group_table_dup");
          } else {
            table_sets.insert(ident.clone());
          }
        } else {
          let ident = (table_ident.schema.clone(), table_ident.ident_alias.clone());
          let ident_string = (ident.0.map(|s| s.to_string), ident.1.to_string);

          if let Some(_) = table_sets.get(&ident_string) {
            panic!("table_group_table_dup");
          } else {
            table_sets.insert(ident_string.clone());
          }
        }
      }

      self
        .table_group_map
        .insert(group_each.ident.to_string.clone(), table_sets);
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

    if let Some(block) = self.schema_map.get(&schema) {
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
    } else {
      panic!("schema_not_found");
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
