use std::str::FromStr;

use self::err::*;
use crate::ast::*;
use crate::DEFAULT_SCHEMA;

mod block;
mod err;
mod indexer;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SemanticSchemaBlock {
  /// Overall description of the project. This is optional. The file must contain one or zero 'Project' block.
  pub project: Option<ProjectBlock>,
  /// Table block.
  pub tables: Vec<TableBlock>,
  /// TableGroup block.
  pub table_groups: Vec<TableGroupBlock>,
  /// Ref block.
  pub refs: Vec<block::IndexedRefBlock>,
  /// Enums block.
  pub enums: Vec<EnumBlock>,
  /// Identifier and alias indexer.
  pub indexer: indexer::Indexer,
}

type TableRefTuple = (
  Vec<block::IndexedRefBlock>,
  Vec<block::IndexedRefBlock>,
  Vec<block::IndexedRefBlock>,
);

impl SemanticSchemaBlock {
  /// Gets a table block's relation (ref to, ref by, ref self).
  pub fn get_table_refs(&self, table_ident: &TableIdent) -> TableRefTuple {
    let mut ref_to_blocks = vec![];
    let mut ref_by_blocks = vec![];
    let mut ref_self_blocks = vec![];

    let eq = |table_ident: &TableIdent, ref_ident: &RefIdent| {
      table_ident.schema == ref_ident.schema && table_ident.name == ref_ident.table
    };

    for ref_block in self.refs.iter() {
      let lhs_ident = self.indexer.resolve_ref_alias(&ref_block.lhs);
      let rhs_ident = self.indexer.resolve_ref_alias(&ref_block.rhs);

      if eq(&table_ident, &lhs_ident) && eq(&table_ident, &rhs_ident) {
        ref_self_blocks.push(ref_block.clone())
      } else if eq(&table_ident, &lhs_ident) {
        ref_to_blocks.push(ref_block.clone())
      } else if eq(&table_ident, &rhs_ident) {
        ref_by_blocks.push(ref_block.clone())
      }
    }

    (ref_to_blocks, ref_by_blocks, ref_self_blocks)
  }
}

impl SchemaBlock<'_> {
  /// Performs semantic checks of the unsanitized AST and returns a sanitized AST.
  pub fn analyze(self) -> AnalyzerResult<SemanticSchemaBlock> {
    let Self {
      span_range,
      input,
      project,
      tables,
      table_groups,
      refs,
      enums,
    } = self;

    // check project block
    match &project {
      Some(project_block) => (),
      _ => throw_err(Err::ProjectSettingNotFound, span_range, input)?,
    }

    // collect tables
    let mut indexer = indexer::Indexer::default();
    let mut indexed_refs: Vec<_> = refs.into_iter().map(block::IndexedRefBlock::from).collect();

    // index inside the table itself
    let tables = tables
      .into_iter()
      .map(|mut table| {
        for col in table.cols.iter() {
          if col.settings.as_ref().is_some_and(|s| s.is_pk) {
            if !table.meta_indexer.pk_list.is_empty() {
              panic!("pk_dup");
            }
            if col.settings.as_ref().is_some_and(|s| matches!(s.is_nullable, Some(Nullable::Null))) {
              panic!("nullable_pk");
            }
            if !col.r#type.arrays.is_empty() {
              panic!("array_pk");
            }

            table.meta_indexer.pk_list.push(col.name.clone())
          }
          if col.settings.as_ref().is_some_and(|s| s.is_unique) {
            table.meta_indexer.unique_list.push(col.name.clone())
          }
        }

        if let Some(indexes_block) = &table.indexes {
          for def in indexes_block.defs.iter() {
            let idents: Vec<_> = def
              .cols
              .iter()
              .filter_map(|id| {
                if let IndexesColumnType::String(s) = id {
                  Some(s)
                } else {
                  None
                }
              })
              .cloned()
              .collect();

            match &def.settings {
              Some(settings) => {
                if settings.is_pk && settings.is_unique {
                  panic!("primary key is already unique");
                }

                if settings.is_pk {
                  if !table.meta_indexer.pk_list.is_empty() {
                    panic!("pk_dup");
                  }

                  table.meta_indexer.pk_list.extend(idents)
                } else if settings.is_unique {
                  // FIXME: furthur validation

                  table.meta_indexer.unique_list.extend(idents)
                }
              }
              None => table.meta_indexer.indexed_list.extend(idents),
            };
          }
        }

        table
      })
      .collect();

    // start indexing the schema
    indexer.index_table(&tables);
    indexer.index_enums(&enums);
    indexer.index_table_groups(&table_groups);

    // collect refs from tables
    for table in &tables {
      for col in &table.cols {
        let indexed_ref = block::IndexedRefBlock::from_inline(
          col.settings.clone().map(|s| s.refs).unwrap_or_else(|| vec![]),
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
        let cols = table
          .cols
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
                    &table.ident.schema,
                    &table.ident.name,
                    &vec![id_string.clone()],
                  )
                  .unwrap_or_else(|x| panic!("{}", x));
              }
            }
          }
        }

        TableBlock { cols, ..table }
      })
      .collect();

    // validate ref
    for indexed_ref in indexed_refs.clone().into_iter() {
      if let Err(msg) = indexed_ref.validate_ref_type(&tables, &indexer) {
        panic!("{}", msg)
      }

      for r in indexed_refs.iter() {
        if r.lhs.compositions.len() != r.rhs.compositions.len() {
          panic!("composite reference must have the same length")
        }
      }

      let count = indexed_refs
        .iter()
        .filter(|other_indexed_ref| indexed_ref.eq(other_indexed_ref, &indexer))
        .count();

      if count != 1 {
        panic!("dedup_relation_decl")
      }
    }

    Ok(SemanticSchemaBlock {
      project,
      tables,
      table_groups,
      refs: indexed_refs,
      enums,
      indexer,
    })
  }
}
