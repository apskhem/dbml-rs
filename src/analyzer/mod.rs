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
    throw_err(Err::DuplicatedProjectSetting, &schema_block.span_range, input)?;
  }
  match project.first() {
    Some(_) => (),
    _ => throw_err(Err::ProjectSettingNotFound, &schema_block.span_range, input)?,
  }

  // index inside the table itself
  for table in &tables {
    let mut tmp_table_indexer = TableIndexer::default();

    for col in &table.cols {
      if let Some(settings) = &col.settings {
        if settings.is_pk {
          if !tmp_table_indexer.pk_list.is_empty() {
            throw_err(Err::DuplicatedPrimaryKey, &col.span_range, input)?;
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
          tmp_table_indexer.unique_list.push(BTreeSet::from([col.name.to_string.clone()]))
        }

        // TODO: handle multiple errs
        let dup_keys = get_attr_duplicate_keys(&settings.attributes);
        if let Some(dup_key) = dup_keys.first() {
          throw_err(Err::DuplicatedAttributeKey, &dup_key.span_range, input)?;
        }

        let filtered: BTreeSet<_> = settings.attributes
          .iter()
          .filter_map(|a| ["not null", "null"].contains(&a.key.to_string.as_str()).then(|| a.key.to_string.clone()))
          .collect();

        if filtered.len() == 2 {
          throw_err(Err::ConflictedNullableSetting, &settings.span_range, input)?;
        }
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
              throw_err(Err::InvalidIndexesSetting, &settings.span_range, input)?;
            }
            
            if settings.is_pk {
              if !tmp_table_indexer.pk_list.is_empty() {
                throw_err(Err::DuplicatedPrimaryKey, &def.span_range, input)?;
              }

              tmp_table_indexer.pk_list.extend(idents.clone())
            } else if settings.is_unique {
              if tmp_table_indexer.unique_list.iter().any(|uniq_item| idents.iter().all(|id| uniq_item.contains(id))) {
                throw_err(Err::DuplicatedUniqueKey, &def.span_range, input)?;
              }

              tmp_table_indexer.unique_list.push(idents.clone().into_iter().collect())
            }

            if settings.r#type.is_some() {
              if tmp_table_indexer.indexed_list.iter().any(|(idx_item, idx_type)| idx_item == &idents && idx_type == &settings.r#type) {
                throw_err(Err::DuplicatedIndexKey, &def.span_range, input)?;
              }

              tmp_table_indexer.indexed_list.push((idents, settings.r#type.clone()));
            }
          }
          None => {
            if tmp_table_indexer.indexed_list.iter().any(|(idx_item, _)| idx_item == &idents) {
              throw_err(Err::DuplicatedIndexKey, &def.span_range, input)?;
            }

            tmp_table_indexer.indexed_list.push((idents, None))
          },
        };
      }
    }
  }

  // collect tables
  let mut indexer = indexer::Indexer::default();
  let mut indexed_refs: Vec<_> = refs.clone().into_iter().cloned().map(block::IndexedRefBlock::from).collect();

  // start indexing the schema
  indexer.index_table(&tables, input)?;
  indexer.index_enums(&enums, input)?;
  indexer.index_table_groups(&table_groups, input)?;

  // collect refs from tables
  for table in &tables {
    for col in &table.cols {
      let indexed_ref = block::IndexedRefBlock::from_inline(
        col.settings.clone().map(|s| s.refs).unwrap_or_default(),
        table.ident.clone(),
        col.name.clone(),
      );

      indexed_refs.extend(indexed_ref);
    }
  }

  // validate table type
  let tables = tables
    .into_iter()
    .map(|table| {
      let cols = table.cols
        .clone()
        .into_iter()
        .map(|col| {
          let type_name = col.r#type.type_name;

          if type_name == ColumnTypeName::Undef {
            unreachable!("undef field type must not appear");
          }

          let type_name = match type_name {
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
                  let default_enum = col.settings
                    .as_ref()
                    .map(|s| s.default.clone().map(|d| vec![d.to_string()]))
                    .flatten()
                    .unwrap_or_default();

                  let splited: Vec<_> = raw_type.split(".").collect();

                  let (enum_schema, enum_name) = match splited.len() {
                    1 => (None, raw_type),
                    2 => (Some(splited[0].to_string()), splited[1].to_string()),
                    _ => throw_err(Err::InvalidEnum, &col.r#type.span_range, input)?,
                  };

                  match indexer.lookup_enum_values(&enum_schema, &enum_name, &default_enum) {
                    Ok(_) => ColumnTypeName::Enum(enum_name),
                    Err(_) => throw_err(Err::InvalidDataType, &col.r#type.span_range, input)?,
                  }
                }
              }
            }
            _ => unreachable!("preprocessing data type name is not raw"),
          };

          // TODO: add more validation
          if let Some(ColumnSettings { attributes, default: Some(default_value), .. }) = col.settings.clone() {
            let span_range = attributes.into_iter().find_map(|attr| {
              (attr.key.to_string == "default").then(|| attr.span_range)
            }).unwrap(); // FIXME: remove unwrap

            // validate default value association with a col type
            match default_value {
              Value::Enum(_) => (),
              Value::String(val) => {
                if !matches!(
                  type_name,
                  ColumnTypeName::Bit
                  | ColumnTypeName::Varbit
                  | ColumnTypeName::Char
                  | ColumnTypeName::VarChar
                  | ColumnTypeName::Enum(_)
                ) {
                  throw_err(Err::InvalidDefaultValue { raw_value: val.clone(), raw_type: col.r#type.raw.clone() }, &span_range, input)?;
                }

                // validate fixed and variable length data type
                match type_name {
                  ColumnTypeName::Bit
                  | ColumnTypeName::Char
                  if matches!(col.r#type.args[0], Value::Integer(len) if val.len() as i64 != len) => {
                    panic!("defualt value does not match with the specified fixed length")
                  }
                  ColumnTypeName::Varbit
                  | ColumnTypeName::VarChar
                  if matches!(col.r#type.args[0], Value::Integer(cap) if val.len() as i64 > cap) => {
                    panic!("defualt value exceeds the specified variable length")
                  }
                  _ => ()
                };
              },
              Value::Integer(val) => {
                if !matches!(
                  type_name,
                  ColumnTypeName::SmallSerial
                  | ColumnTypeName::Serial
                  | ColumnTypeName::BigSerial
                  | ColumnTypeName::SmallInt
                  | ColumnTypeName::Integer
                  | ColumnTypeName::BigInt
                ) {
                  throw_err(Err::InvalidDefaultValue { raw_value: val.to_string(), raw_type: col.r#type.raw.clone() }, &span_range, input)?;
                }

                match type_name {
                  ColumnTypeName::SmallInt
                  if (val > i16::MAX as i64) || (val < i16::MIN as i64) => {
                    panic!("defualt value exceeds the maximum value of the specified type")
                  }
                  ColumnTypeName::Integer
                  if (val > i32::MAX as i64) || (val < i32::MIN as i64) => {
                    panic!("defualt value exceeds the maximum value of the specified type")
                  }
                  ColumnTypeName::BigInt
                  if val.overflowing_add(1).1 || val.overflowing_sub(1).1 => {
                    panic!("defualt value exceeds the maximum value of the specified type")
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

      if let Some(block) = &table.indexes {
        for def in block.defs.iter() {
          if def.cols.is_empty() {
            panic!("indexes def (..) cannot be empty")
          }

          for ident in def.cols.iter() {
            if let IndexesColumnType::String(col_name) = ident {
              indexer
                .lookup_table_fields(
                  &table.ident.schema,
                  &table.ident.name,
                  &vec![Ident { span_range: 0..0, raw: String::new(), to_string: col_name.clone() }],
                )
                .unwrap_or_else(|x| panic!("{}", x));
            }
          }
        }
      }

      Ok(TableBlock { cols, ..table.clone() })
    })
    .collect::<AnalyzerResult<_>>()?;

  // validate ref
  for indexed_ref in indexed_refs.clone().into_iter() {
    indexed_ref.validate_ref_type(&tables, &indexer, input)?;

    for r in indexed_refs.iter() {
      if r.lhs.compositions.len() != r.rhs.compositions.len() {
        throw_err(Err::MismatchedCompositeForeignKey, &indexed_ref.span_range, input)?;
      }
    }

    let count = indexed_refs
      .iter()
      .filter(|other_indexed_ref| indexed_ref.eq(other_indexed_ref, &indexer))
      .count();

    if count != 1 {
      throw_err(Err::DuplicatedRelation, &indexed_ref.span_range, input)?;
    }
  }

  Ok(AnalyzedIndexer {
    indexed_refs,
    indexer
  })
}

fn get_attr_duplicate_keys(attrs: &Vec<Attribute>) -> Vec<Attribute> {
  let mut sum = BTreeSet::new();
  let mut dup = vec![];
  for attr in attrs {
    if sum.contains(&attr.key.to_string) {
      dup.push(attr.clone())
    } else {
      sum.insert(attr.key.to_string.clone());
    }
  }
  dup
}