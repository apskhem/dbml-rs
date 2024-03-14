use alloc::collections::BTreeSet;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::str::FromStr;

use self::err::*;
use crate::ast::*;
use crate::DEFAULT_SCHEMA;

mod err;
mod helper;
mod indexer;

use helper::*;
use indexer::*;

#[derive(Debug, Clone, Default)]
pub struct AnalyzedIndexer {
  pub indexed_refs: Vec<IndexedRef>,
  pub indexer: Indexer,
}

/// Represents a reference to a table, indicating relationships between tables.
pub struct TableRef {
  /// References that this table points to, such as foreign keys in its fields.
  pub ref_to: Vec<IndexedRef>,
  /// References to this table, indicating other tables that have foreign keys pointing to it.
  pub ref_by: Vec<IndexedRef>,
  /// Self-references within this table.
  pub ref_self: Vec<IndexedRef>,
}

/// Performs semantic checks of the unsanitized AST and returns an indexed metadata.
/// This function also mutates the internal structure of the AST by changing column types after validating.
///
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
  let input = schema_block.input;
  let project = schema_block.project();
  let tables = schema_block.tables();
  let table_groups = schema_block.table_groups();
  let refs = schema_block.refs();
  let enums = schema_block.enums();

  // check project block
  if project.len() > 1 {
    throw_err(Err::DuplicateProjectSetting, &schema_block.span_range, input)?;
  }
  match project.first() {
    Some(project_block) => {
      check_prop_duplicate_keys(&project_block.properties, input)?;
    }
    _ => throw_err(Err::ProjectSettingNotFound, &schema_block.span_range, input)?,
  }

  // collect tables
  let mut indexer = Indexer::default();
  let mut indexed_refs: Vec<_> = refs.into_iter().cloned().map(IndexedRef::from).collect();

  // start indexing the schema
  indexer.index_table(&tables, input)?;
  indexer.index_enums(&enums, input)?;
  indexer.index_table_groups(&table_groups, input)?;

  // index inside the table itself
  for table in &tables {
    let mut tmp_table_indexer = TableIndexer::default();

    // validate columns
    for col in &table.cols {
      if let Some(settings) = &col.settings {
        if settings.is_pk {
          if !tmp_table_indexer.pk_list.is_empty() {
            throw_err(Err::DuplicatePrimaryKey, &col.span_range, input)?;
          }
          if settings.nullable == Some(Nullable::Null) {
            throw_err(Err::NullablePrimaryKey, &col.span_range, input)?;
          }
          if !col.r#type.arrays.is_empty() {
            throw_err(Err::ArrayPrimaryKey, &col.span_range, input)?;
          }

          tmp_table_indexer.pk_list.push(col.name.to_string.clone())
        }
        if settings.is_unique {
          tmp_table_indexer
            .unique_list
            .push(BTreeSet::from([col.name.to_string.clone()]))
        }

        let filtered: BTreeSet<_> = settings
          .attributes
          .iter()
          .filter(|&a| ["not null", "null"].contains(&a.key.to_string.as_str()))
          .map(|a| &a.key.to_string)
          .collect();

        if filtered.len() == 2 {
          throw_err(Err::ConflictNullableSetting, &settings.span_range, input)?;
        }
      }

      // collect refs from columns
      let indexed_ref = IndexedRef::from_inline(
        col.settings.clone().map(|s| s.refs).unwrap_or_default(),
        &table.ident,
        &col.name,
      );

      indexed_refs.extend(indexed_ref);
    }

    // validate indexes block
    if let Some(indexes_block) = &table.indexes {
      for def in &indexes_block.defs {
        if def.cols.is_empty() {
          throw_err(Err::EmptyIndexesBlock, &indexes_block.span_range, input)?;
        }

        let idents: Vec<_> = def
          .cols
          .iter()
          .filter_map(|id| {
            match id {
              IndexesColumnType::String(s) => Some(s),
              _ => None,
            }
          })
          .cloned()
          .collect();
        let ident_strings: Vec<_> = idents.iter().map(|s| s.to_string.clone()).collect();

        for ident in &def.cols {
          if let IndexesColumnType::String(col_name) = ident {
            if !table.cols.iter().any(|col| col.name.to_string == col_name.to_string) {
              throw_err(Err::ColumnNotFound, &col_name.span_range, input)?;
            }
          }
        }

        match &def.settings {
          Some(settings) => {
            check_attr_duplicate_keys(&settings.attributes, input)?;

            if vec![settings.is_pk, settings.is_unique, settings.r#type.is_some()]
              .into_iter()
              .filter(|x| *x)
              .count()
              > 1
            {
              throw_err(Err::InvalidIndexesSetting, &settings.span_range, input)?;
            }

            if settings.is_pk {
              if !tmp_table_indexer.pk_list.is_empty() {
                throw_err(Err::DuplicatePrimaryKey, &def.span_range, input)?;
              }

              tmp_table_indexer.pk_list.extend(ident_strings.clone())
            } else if settings.is_unique {
              if tmp_table_indexer
                .unique_list
                .iter()
                .any(|uniq_item| idents.iter().all(|id| uniq_item.contains(&id.to_string)))
              {
                throw_err(Err::DuplicateUniqueKey, &def.span_range, input)?;
              }

              tmp_table_indexer
                .unique_list
                .push(ident_strings.clone().into_iter().collect())
            }

            if settings.r#type.is_some() {
              if tmp_table_indexer
                .indexed_list
                .iter()
                .any(|(idx_item, idx_type)| idx_item == &ident_strings && idx_type == &settings.r#type)
              {
                throw_err(Err::DuplicateIndexKey, &def.span_range, input)?;
              }

              tmp_table_indexer
                .indexed_list
                .push((ident_strings, settings.r#type.clone()));
            }
          }
          None => {
            if tmp_table_indexer
              .indexed_list
              .iter()
              .any(|(idx_item, _)| idx_item == &ident_strings)
            {
              throw_err(Err::DuplicateIndexKey, &def.span_range, input)?;
            }

            tmp_table_indexer.indexed_list.push((ident_strings, None))
          }
        };
      }
    }
  }

  // validate table column types
  let tables = tables
    .into_iter()
    .map(|table| {
      let cols = table.cols
        .clone()
        .into_iter()
        .map(|col| {
          let type_name = col.r#type.type_name;

          let type_name = match type_name {
            ColumnTypeName::Undef => {
              unreachable!("undef field type must not appear");
            }
            ColumnTypeName::Raw(raw_type) => {
              match ColumnTypeName::from_str(&raw_type) {
                Ok(type_name) => {
                  if !col.r#type.args.is_empty() {
                    // TODO: add support for interval
                    match type_name {
                      ColumnTypeName::VarChar
                      | ColumnTypeName::Char
                      | ColumnTypeName::Time
                      | ColumnTypeName::Timestamp
                      | ColumnTypeName::Timetz
                      | ColumnTypeName::Timestamptz
                      | ColumnTypeName::Bit
                      | ColumnTypeName::Varbit => {
                        if col.r#type.args.len() != 1 {
                          throw_err(Err::InvalidDataTypeArguments { raw_type, n_arg: 1 }, &col.r#type.span_range, input)?;
                        }
                      }
                      ColumnTypeName::Decimal => {
                        if col.r#type.args.len() != 2 {
                          throw_err(Err::InvalidDataTypeArguments { raw_type, n_arg: 2 }, &col.r#type.span_range, input)?;
                        }
                      }
                      _ =>  {
                        throw_err(Err::InvalidDataTypeArguments { raw_type, n_arg: 0 }, &col.r#type.span_range, input)?
                      },
                    };

                    if !col.r#type.args.iter().all(|arg| matches!(arg, Value::Integer(_))) {
                      throw_err(Err::InvalidArgumentValue, &col.r#type.span_range, input)?;
                    }
                  }
                  
                  type_name
                }
                Err(_) => {
                  let splited: Vec<_> = raw_type.split('.').collect();

                  let (enum_schema, enum_name) = match splited.len() {
                    1 => (None, raw_type),
                    2 => (Some(splited[0].to_string()), splited[1].to_string()),
                    _ => throw_err(Err::InvalidEnum, &col.r#type.span_range, input)?,
                  };

                  match &col.settings {
                    Some(ColumnSettings { attributes, default: Some(default_value), .. }) => {
                      let default_value_span = attributes.iter()
                        .find_map(|attr| {
                          (attr.key.to_string == "default").then(|| attr.value.as_ref().map(|v| &v.span_range))
                        })
                        .and_then(|opt_span| opt_span)
                        .unwrap_or_else(|| unreachable!("default value is missing"));

                      match indexer.lookup_enum_values(&enum_schema, &enum_name, &vec![default_value.to_string()]) {
                        (false, (_, _)) => throw_err(Err::SchemaNotFound, &col.r#type.span_range, input)?,
                        (true, (false, _)) => throw_err(Err::EnumNotFound, &col.r#type.span_range, input)?,
                        (true, (true, f)) if f.iter().any(|f| f == &false) => throw_err(Err::EnumValueNotFound, &default_value_span, input)?,
                        _ => ColumnTypeName::Enum(enum_name)
                      }
                    }
                    _ => {
                      ColumnTypeName::Enum(enum_name)
                    }
                  }
                }
              }
            }
            _ => unreachable!("preprocessing data type name is not raw"),
          };

          // TODO: add more validation
          if let Some(ColumnSettings { attributes, default: Some(default_value), .. }) = &col.settings {
            let span_range = attributes.iter()
              .find_map(|attr| {
                (attr.key.to_string == "default").then(|| attr.value.as_ref().map(|v| &v.span_range))
              })
              .and_then(|opt_span| opt_span)
              .unwrap_or_else(|| unreachable!("default value is missing"));

            // validate default value association with a col type
            match default_value {
              Value::Enum(_) => (),
              Value::String(val) => {
                let err = Err::InvalidDefaultValue { raw_value: val.clone(), raw_type: col.r#type.raw.clone() };

                // TODO: validate which type can be strings

                // validate fixed and variable length data type
                match type_name {
                  ColumnTypeName::Bit
                  | ColumnTypeName::Char
                  if matches!(col.r#type.args[0], Value::Integer(len) if val.len() as i64 != len) => {
                    throw_err(err.clone(), &span_range, input)?;
                  }
                  ColumnTypeName::Varbit
                  | ColumnTypeName::VarChar
                  if matches!(col.r#type.args[0], Value::Integer(cap) if val.len() as i64 > cap) => {
                    throw_err(err.clone(), &span_range, input)?;
                  }
                  _ => ()
                };
              },
              Value::Integer(val) => {
                let err = Err::DataTypeExceeded { raw_type: col.r#type.raw.clone() };

                // TODO: validate which type can be numbers

                match type_name {
                  ColumnTypeName::SmallInt
                  if (*val > i16::MAX as i64) || (*val < i16::MIN as i64) => {
                    throw_err(err.clone(), &span_range, input)?;
                  }
                  ColumnTypeName::Integer
                  if (*val > i32::MAX as i64) || (*val < i32::MIN as i64) => {
                    throw_err(err.clone(), &span_range, input)?;
                  }
                  ColumnTypeName::BigInt
                  if val.overflowing_add(1).1 || val.overflowing_sub(1).1 => {
                    throw_err(err.clone(), &span_range, input)?;
                  }
                  _ => ()
                };
              },
              Value::Decimal(_) => (),
              Value::Bool(val) => {
                if ![ColumnTypeName::Bool].contains(&type_name) {
                  throw_err(Err::InvalidDefaultValue { raw_value: val.to_string(), raw_type: col.r#type.raw.clone() }, &span_range, input)?;
                }
              }
              Value::HexColor(_) => (),
              Value::Expr(_) => (),
              Value::Null => {
                if !col.settings.as_ref().is_some_and(|s| s.nullable == Some(Nullable::Null)) {
                  throw_err(Err::DefaultNullInNonNullable, &span_range, input)?;
                }
              }
            }
          }

          Ok(TableColumn {
            r#type: ColumnType {
              type_name,
              ..col.r#type
            },
            ..col
          })
        })
        .collect::<AnalyzerResult<_>>()?;

      Ok(TableBlock { cols, ..table.clone() })
    })
    .collect::<AnalyzerResult<_>>()?;

  // validate ref
  for indexed_ref in &indexed_refs {
    if let Some(settings) = &indexed_ref.settings {
      check_attr_duplicate_keys(&settings.attributes, input)?;
    }

    indexed_ref.validate_ref_type(&tables, &indexer, input)?;

    let count = indexed_refs
      .iter()
      .filter(|other_indexed_ref| indexed_ref.occupy_same_column(other_indexed_ref, &indexer))
      .count();

    if count != 1 {
      throw_err(Err::ConflictRelation, &indexed_ref.span_range, input)?;
    }
  }

  Ok(AnalyzedIndexer { indexed_refs, indexer })
}
