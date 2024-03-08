use alloc::collections::BTreeSet;

use std::str::FromStr;

use self::err::*;
use crate::ast::*;
use crate::DEFAULT_SCHEMA;

mod block;
mod err;
mod indexer;

#[derive(Debug, Clone, Default)]
pub struct AnalyzedIndexer {
  pub indexed_refs: Vec<block::IndexedRefBlock>,
  pub indexer: indexer::Indexer,
}

/// Represents a reference to a table, indicating relationships between tables.
pub struct TableRef {
  /// References that this table points to, such as foreign keys in its fields.
  pub ref_to: Vec<block::IndexedRefBlock>,
  /// References to this table, indicating other tables that have foreign keys pointing to it.
  pub ref_by: Vec<block::IndexedRefBlock>,
  /// Self-references within this table.
  pub ref_self: Vec<block::IndexedRefBlock>,
}

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
    table_ident.schema.clone().map(|s| s.to_string) == ref_ident.schema && table_ident.name.to_string == ref_ident.table
  };

  for ref_block in analyzed_indexer.indexed_refs.iter() {
    let lhs_ident = analyzed_indexer.indexer.resolve_ref_alias(&ref_block.lhs);
    let rhs_ident = analyzed_indexer.indexer.resolve_ref_alias(&ref_block.rhs);

    if eq(&table_ident, &lhs_ident) && eq(&table_ident, &rhs_ident) {
      ref_self_blocks.push(ref_block.clone())
    } else if eq(&table_ident, &lhs_ident) {
      ref_to_blocks.push(ref_block.clone())
    } else if eq(&table_ident, &rhs_ident) {
      ref_by_blocks.push(ref_block.clone())
    }
  }

  TableRef {
    ref_to: ref_to_blocks,
    ref_by: ref_by_blocks,
    ref_self: ref_self_blocks
  }
}

/// Performs semantic checks of the unsanitized Abstract Syntax Tree (AST) and returns a sanitized AST.
///
/// # Arguments
///
/// * `schema_block` - A reference to the unsanitized AST representing the parsed DBML text.
///
/// # Returns
///
/// An `AnalyzerResult<AnalyzedIndexer>`, which is an alias for the result of semantic analysis.
/// It contains the indexing metadata for the collecting table relations and block names representing the parsed and analyzed DBML.
///
/// # Examples
///
/// ```rs
/// use dbml_rs::{parse_dbml, analyze};
///
/// let dbml_text = r#"
///     Table users {
///         id int
///         username varchar
///     }
/// "#;
///
/// let result = parse_dbml(dbml_text);
/// assert!(result.is_ok());
/// let ast = result.unwrap();
///
/// let analyzing_result = analyze(&ast);
/// // Now we can guarantee that `ast` is sanitized and semantically checked.
/// assert!(analyzing_result.is_ok());
/// let analyzed_indexer = analyzing_result.unwrap();
/// // of the parsed and analyzed DBML text.
/// ```
pub fn analyze(schema_block: &SchemaBlock) -> AnalyzerResult<AnalyzedIndexer> {
  let SchemaBlock {
    span_range,
    input,
    project,
    tables,
    table_groups,
    refs,
    enums,
  } = schema_block;

  // check project block
  match &project {
    Some(project_block) => (),
    _ => throw_err(Err::ProjectSettingNotFound, span_range.clone(), input)?,
  }

  // index inside the table itself
  for table in tables {
    let mut tmp_table_indexer = TableIndexer::default();

    for col in &table.cols {
      if col.settings.as_ref().is_some_and(|s| s.is_pk) {
        if !tmp_table_indexer.pk_list.is_empty() {
          throw_err(Err::DuplicatedPrimaryKey, col.span_range.clone(), input)?;
        }
        if col.settings.as_ref().is_some_and(|s| matches!(s.is_nullable, Some(Nullable::Null))) {
          throw_err(Err::NullablePrimaryKey, col.span_range.clone(), input)?;
        }
        if !col.r#type.arrays.is_empty() {
          throw_err(Err::ArrayPrimaryKey, col.span_range.clone(), input)?;
        }

        tmp_table_indexer.pk_list.push(col.name.to_string.clone())
      }
      if col.settings.as_ref().is_some_and(|s| s.is_unique) {
        tmp_table_indexer.unique_list.push(BTreeSet::from([col.name.to_string.clone()]))
      }
    }

    if let Some(indexes_block) = &table.indexes {
      for def in &indexes_block.defs {
        let idents: Vec<_> = def
          .cols
          .iter()
          .filter_map(|id| {
            match id {
              IndexesColumnType::String(s) => Some(s),
              _ => None
            }
          })
          .cloned()
          .collect();

        match &def.settings {
          Some(settings) => {
            if vec![settings.is_pk, settings.is_unique, settings.r#type.is_some()].into_iter().filter(|x| *x).count() > 1 {
              throw_err(Err::InvalidIndexesSetting, settings.span_range.clone(), input)?;
            }
            
            if settings.is_pk {
              if !tmp_table_indexer.pk_list.is_empty() {
                throw_err(Err::DuplicatedPrimaryKey, def.span_range.clone(), input)?;
              }

              tmp_table_indexer.pk_list.extend(idents.clone())
            } else if settings.is_unique {
              if tmp_table_indexer.unique_list.iter().any(|uniq_item| idents.iter().all(|id| uniq_item.contains(id))) {
                throw_err(Err::DuplicatedUniqueKey, def.span_range.clone(), input)?;
              }

              tmp_table_indexer.unique_list.push(idents.clone().into_iter().collect())
            }

            if settings.r#type.is_some() {
              if tmp_table_indexer.indexed_list.iter().any(|(idx_item, idx_type)| idx_item == &idents && idx_type == &settings.r#type) {
                throw_err(Err::DuplicatedIndexKey, def.span_range.clone(), input)?;
              }

              tmp_table_indexer.indexed_list.push((idents, settings.r#type.clone()));
            }
          }
          None => {
            if tmp_table_indexer.indexed_list.iter().any(|(idx_item, _)| idx_item == &idents) {
              throw_err(Err::DuplicatedIndexKey, def.span_range.clone(), input)?;
            }

            tmp_table_indexer.indexed_list.push((idents, None))
          },
        };
      }
    }
  }

  // collect tables
  let mut indexer = indexer::Indexer::default();
  let mut indexed_refs: Vec<_> = refs.clone().into_iter().map(block::IndexedRefBlock::from).collect();

  // start indexing the schema
  indexer.index_table(&tables, &input)?;
  indexer.index_enums(&enums)?;
  indexer.index_table_groups(&table_groups, &input)?;

  // collect refs from tables
  for table in tables {
    for col in &table.cols {
      let indexed_ref = block::IndexedRefBlock::from_inline(
        col.settings.clone().map(|s| s.refs).unwrap_or_default(),
        table.ident.clone(),
        col.name.to_string.clone(),
      );

      indexed_refs.extend(indexed_ref);
    }
  }

  // validate table type
  let tables = tables
    .into_iter()
    .map(|table| {
      let cols = table
        .cols
        .clone()
        .into_iter()
        .map(|col| {
          let type_name = col.r#type.type_name;

          if type_name == ColumnTypeName::Undef {
            panic!("undef_table_field")
          }

          let type_name = match type_name {
            ColumnTypeName::Raw(raw_type) => {
              match ColumnTypeName::from_str(&raw_type) {
                Ok(type_name) => {
                  if col.r#type.args.is_empty() {
                    type_name
                  } else {
                    // validate args (if has)
                    match type_name {
                      ColumnTypeName::VarChar | ColumnTypeName::Char => {
                        if col.r#type.args.len() != 1 {
                          panic!("varchar_incompatible_args")
                        }

                        col
                          .r#type
                          .args
                          .iter()
                          .fold(type_name, |acc, arg| match arg {
                            Value::Integer(_) => acc,
                            _ => panic!("varchar_args_is_not_integer"),
                          })
                      }
                      ColumnTypeName::Decimal => {
                        if col.r#type.args.len() != 2 {
                          panic!("decimal_incompatible_args")
                        }

                        col
                          .r#type
                          .args
                          .iter()
                          .fold(type_name, |acc, arg| match arg {
                            Value::Integer(_) => acc,
                            _ => panic!("decimal_args_is_not_integer"),
                          })
                      }
                      _ => panic!("invalid args usage"),
                    }
                  }
                }
                Err(_) => {
                  let default_enum = match col.settings.as_ref().map(|s| s.default.clone()) {
                    Some(Some(default)) => vec![default.to_string()],
                    _ => vec![],
                  };

                  let splited: Vec<_> = raw_type.split(".").collect();

                  let (enum_schema, enum_name) = match splited.len() {
                    2 => (Some(splited[0].to_string()), splited[1].to_string()),
                    1 => (None, raw_type),
                    _ => panic!("incorrect enum field format"),
                  };

                  match indexer.lookup_enum_values(&enum_schema, &enum_name, &default_enum) {
                    Ok(_) => ColumnTypeName::Enum(enum_name),
                    Err(msg) => panic!("'{}' is an invalid type", msg),
                  }
                }
              }
            }
            _ => panic!("preprecessing_type_is_not_raw"),
          };

          // TODO: add more validation
          if let Some(Some(default_value)) = col.settings.as_ref().map(|s| s.default.clone()) {
            match default_value {
              Value::String(_) => (),
              Value::Integer(_) => (),
              Value::Decimal(_) => (),
              Value::Bool(_) => {
                if ![ColumnTypeName::Bool].contains(&type_name) {
                  panic!("defualt value is not associated with declared type")
                }
              }
              Value::HexColor(_) => (),
              Value::Expr(_) => (),
              Value::Null => {
                if !col.settings.as_ref().is_some_and(|s| matches!(s.is_nullable, Some(Nullable::Null))) {
                  panic!("default value cannot be null in non-nullable field")
                }
              }
            }
          }

          TableColumn {
            r#type: ColumnType {
              type_name,
              ..col.r#type
            },
            ..col
          }
        })
        .collect();

      if let Some(block) = &table.indexes {
        for def in block.defs.iter() {
          if def.cols.is_empty() {
            panic!("indexes def (..) cannot be empty")
          }

          for ident in def.cols.iter() {
            if let IndexesColumnType::String(id_string) = ident {
              indexer
                .lookup_table_fields(
                  &table.ident.schema.clone().map(|s| s.to_string),
                  &table.ident.name.to_string,
                  &vec![id_string.clone()],
                )
                .unwrap_or_else(|x| panic!("{}", x));
            }
          }
        }
      }

      TableBlock { cols, ..table.clone() }
    })
    .collect();

  // validate ref
  for indexed_ref in indexed_refs.clone().into_iter() {
    indexed_ref.validate_ref_type(&tables, &indexer, &input)?;

    for r in indexed_refs.iter() {
      if r.lhs.compositions.len() != r.rhs.compositions.len() {
        throw_err(Err::MismatchedCompositeForeignKey, indexed_ref.span_range.clone(), &input)?;
      }
    }

    let count = indexed_refs
      .iter()
      .filter(|other_indexed_ref| indexed_ref.eq(other_indexed_ref, &indexer))
      .count();

    if count != 1 {
      throw_err(Err::DuplicatedRelation, indexed_ref.span_range.clone(), &input)?;
    }
  }

  Ok(AnalyzedIndexer {
    indexed_refs,
    indexer
  })
}

