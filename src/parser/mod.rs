mod err;
mod helper;

use std::str::FromStr;

use err::*;
use pest::iterators::Pair;
use pest::Parser;

use crate::ast::*;
use self::helper::*;

#[derive(Parser)]
#[grammar = "src/dbml.pest"]
struct DBMLParser;

/// Parses the entire DBML text and returns an unsanitized Abstract Syntax Tree (AST).
///
/// # Arguments
///
/// * `input` - A string slice containing the DBML text to parse.
///
/// # Returns
///
/// A `ParserResult<SchemaBlock>`, which is an alias for `pest`'s `ParseResult` type
/// representing the result of parsing. It contains the unsanitized abstract syntax tree (AST)
/// representing the parsed DBML.
///
/// # Errors
///
/// This function can return parsing errors if the input text does not conform to the DBML grammar.
/// It may also panic if an unexpected parsing rule is encountered, which should be considered a
/// bug.
///
/// # Examples
///
/// ```rs
/// use dbml_rs::parse_dbml_unchecked;
///
/// let dbml_text = r#"
///     Table users {
///         id int
///         username varchar
///     }
/// "#;
///
/// let result = parse_dbml_unchecked(dbml_text);
/// assert!(result.is_ok());
/// let ast = result.unwrap();
/// // Now `ast` contains the unsanitized abstract syntax tree (AST) of the parsed DBML text.
/// ```
pub fn parse(input: &str) -> ParserResult<SchemaBlock> {
  let pair = DBMLParser::parse(Rule::schema, input)?
    .next()
    .ok_or_else(|| unreachable!("unhandled parsing error"))?;

  match pair.as_rule() {
    Rule::schema => Ok(parse_schema(pair, input)?),
    _ => throw_rules(&[Rule::schema], pair)?,
  }
}

fn parse_schema<'a>(pair: Pair<Rule>, input: &'a str) -> ParserResult<SchemaBlock<'a>> {
  let init = SchemaBlock {
    span_range: s2r(pair.as_span()),
    input: &input,
    ..Default::default()
  };

  pair.into_inner().try_fold(init, |mut acc, p1| {
    match p1.as_rule() {
      Rule::project_decl => {
        acc.blocks.push(TopLevelBlock::Project(parse_project_decl(p1)?))
      },
      Rule::table_decl => {
        acc.blocks.push(TopLevelBlock::Table(parse_table_decl(p1)?))
      },
      Rule::enum_decl => {
        acc.blocks.push(TopLevelBlock::Enum(parse_enum_decl(p1)?))
      },
      Rule::ref_decl => {
        acc.blocks.push(TopLevelBlock::Ref(parse_ref_decl(p1)?))
      },
      Rule::table_group_decl => {
        acc.blocks.push(TopLevelBlock::TableGroup(parse_table_group_decl(p1)?))
      },
      Rule::EOI => (),
      _ => throw_rules(
        &[
          Rule::project_decl,
          Rule::table_decl,
          Rule::enum_decl,
          Rule::ref_decl,
          Rule::table_group_decl,
        ],
        p1,
      )?,
    };

    Ok(acc)
  })
}

fn parse_project_decl(pair: Pair<Rule>) -> ParserResult<ProjectBlock> {
  let init = ProjectBlock {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair.into_inner().try_fold(init, |mut acc, p1| {
    match p1.as_rule() {
      Rule::ident => {
        acc.ident = parse_ident(p1)?
      },
      Rule::project_block => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::property => {
              let prop = parse_property(p2.clone())?;

              match prop.key.to_string.as_str() {
                "database_type" => {
                  if let Value::String(db_name) = prop.value.value.clone() {
                    acc.database_type = match DatabaseType::from_str(&db_name) {
                      Ok(val) => val,
                      Err(msg) => throw_msg(msg, p2)?,
                    }
                  }
                }
                _ => (),
              }

              acc.properties.push(prop)
            }
            Rule::note_decl => {
              acc.note = Some(parse_note_decl(p2)?)
            },
            _ => {
              throw_rules(&[Rule::property, Rule::note_decl], p2)?
            },
          };
        }
      },
      _ => throw_rules(&[Rule::project_block], p1)?,
    }

    Ok(acc)
  })
}

fn parse_table_decl(pair: Pair<Rule>) -> ParserResult<TableBlock> {
  let init = TableBlock {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      

      match p1.as_rule() {
        Rule::decl_ident => {
          acc.ident.span_range = s2r(p1.as_span());

          let (schema, name) = parse_decl_ident(p1)?;

          acc.ident.name = name;
          acc.ident.schema = schema;
        }
        Rule::table_alias => {
          let ident = p1.into_inner().next().unwrap();
          acc.ident.alias = Some(parse_ident(ident)?)
        },
        Rule::table_block => {
          for p2 in p1.into_inner() {
            match p2.as_rule() {
              Rule::table_col => acc.cols.push(parse_table_col(p2)?),
              Rule::note_decl => acc.note = Some(parse_note_decl(p2)?),
              Rule::indexes_decl => acc.indexes = Some(parse_indexes_decl(p2)?),
              _ => throw_rules(&[Rule::table_col, Rule::note_decl, Rule::indexes_decl], p2)?,
            }
          }
        },
        Rule::block_settings => {
          acc.settings = Some(parse_block_settings(p1)?);
        }
        _ => throw_rules(
          &[
            Rule::decl_ident,
            Rule::table_alias,
            Rule::table_block,
            Rule::block_settings,
          ],
          p1,
        )?,
      }

      Ok(acc)
    })
}

fn parse_block_settings(pair: Pair<Rule>) -> ParserResult<TableSettings> {
  let mut init = TableSettings {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  init.attributes = pair
    .into_inner()
    .map(|p1| {
      match p1.as_rule() {
        Rule::attribute => parse_attribute(p1),
        _ => throw_rules(&[Rule::attribute], p1),
      }
    })
    .collect::<ParserResult<_>>()?;

  Ok(init)
}

fn parse_table_col(pair: Pair<Rule>) -> ParserResult<TableColumn> {
  let init = TableColumn {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::ident => {
          acc.name = parse_ident(p1)?
        },
        Rule::col_type => {
          acc.r#type = parse_col_type(p1)?;
        }
        Rule::col_settings => {
          acc.settings = Some(parse_col_settings(p1)?)
        },
        _ => throw_rules(&[Rule::ident, Rule::col_type, Rule::col_settings], p1)?,
      }

      Ok(acc)
    })
}

fn parse_col_type(pair: Pair<Rule>) -> ParserResult<ColumnType> {
  let mut out = ColumnType {
    span_range: s2r(pair.as_span()),
    raw: pair.as_str().to_string(),
    ..Default::default()
  };

  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::col_type_quoted | Rule::col_type_unquoted => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::var | Rule::spaced_var => {
              out.type_name = ColumnTypeName::Raw(p2.as_str().to_string())
            }
            Rule::col_type_arg => out.args = parse_col_type_arg(p2)?,
            Rule::col_type_array => {
              let val = p2.into_inner().try_fold(None, |_, p3| match p3.as_rule() {
                Rule::integer => {
                  let val = match p3.as_str().parse::<u32>() {
                    Ok(val) => Some(val),
                    Err(err) => throw_msg(err.to_string(), p3)?,
                  };

                  Ok(val)
                }
                _ => throw_rules(&[Rule::integer], p3)?,
              })?;

              out.arrays.push(val)
            }
            _ => throw_rules(
              &[
                Rule::var,
                Rule::spaced_var,
                Rule::col_type_arg,
                Rule::col_type_array,
              ],
              p2,
            )?,
          }
        }
      }
      _ => throw_rules(&[Rule::col_type_quoted, Rule::col_type_unquoted], p1)?,
    }
  }

  Ok(out)
}

fn parse_col_type_arg(pair: Pair<Rule>) -> ParserResult<Vec<Value>> {
  pair.into_inner().try_fold(vec![], |mut acc, p1| {
    match p1.as_rule() {
      Rule::value => acc.push(parse_value(p1)?),
      _ => throw_rules(&[Rule::value], p1)?,
    }

    Ok(acc)
  })
}

fn parse_col_settings(pair: Pair<Rule>) -> ParserResult<ColumnSettings> {
  let init = ColumnSettings {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };
    
  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::col_attribute => {
          for p2 in p1.into_inner() {
            match p2.as_rule() {
              Rule::attribute => {
                let attr = parse_attribute(p2)?;

                match attr.key.to_string.as_str() {
                  "unique" => acc.is_unique = true,
                  "primary key" | "pk" => acc.is_pk = true,
                  "null" => acc.is_nullable = Some(Nullable::Null),
                  "not null" => acc.is_nullable = Some(Nullable::NotNull),
                  "increment" => acc.is_incremental = true,
                  "default" =>  acc.default = attr.value.clone().map(|v| v.value),
                  "note" => acc.note = attr.value.clone().map(|v| v.value.to_string()),
                  _ => ()
                }

                acc.attributes.push(attr);
              },
              Rule::ref_inline => {
                acc.refs.push(parse_ref_inline(p2)?)
              },
              _ => throw_rules(
                &[
                  Rule::ref_inline,
                  Rule::attribute,
                ],
                p2,
              )?,
            }
          }
        }
        _ => throw_rules(&[Rule::col_attribute], p1)?,
      }

      Ok(acc)
    })
}

fn parse_enum_decl(pair: Pair<Rule>) -> ParserResult<EnumBlock> {
  let init = EnumBlock {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::decl_ident => {
          acc.ident.span_range = s2r(p1.as_span());

          let (schema, name) = parse_decl_ident(p1)?;

          acc.ident.schema = schema;
          acc.ident.name = name;
        }
        Rule::enum_block => {
          acc.values = parse_enum_block(p1)?
        },
        _ => {
          throw_rules(&[Rule::decl_ident, Rule::enum_block], p1)?
        },
      }

      Ok(acc)
    })
}

fn parse_enum_block(pair: Pair<Rule>) -> ParserResult<Vec<EnumValue>> {
  pair
    .into_inner()
    .into_iter()
    .map(|p1| {
      match p1.as_rule() {
        Rule::enum_value => Ok(parse_enum_value(p1)?),
        _ => throw_rules(&[Rule::enum_value], p1)?,
      }
    })
    .collect()
}

fn parse_enum_value(pair: Pair<Rule>) -> ParserResult<EnumValue> {
  let init = EnumValue {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::ident => acc.value = parse_ident(p1)?,
        Rule::enum_settings => {
          for p2 in p1.into_inner() {
            match p2.as_rule() {
              Rule::attribute => {
                let attr = parse_attribute(p2)?;

                match attr.key.to_string.as_str() {
                  "note" => acc.note = attr.value.clone().map(|v| v.value.to_string()),
                  _ => ()
                }

                acc.attributes.push(attr);
              }
              _ => throw_rules(&[Rule::attribute], p2)?,
            }
          }
        }
        _ => throw_rules(&[Rule::ident, Rule::enum_settings], p1)?,
      }

      Ok(acc)
    })
}

fn parse_ref_decl(pair: Pair<Rule>) -> ParserResult<RefBlock> {
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ref_block | Rule::ref_short => {
        let mut name = None;

        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::ref_stmt => {
              return parse_ref_stmt(p2).map(|mut o| {
                o.name = name;
                o
              });
            },
            Rule::ident => {
              name = Some(parse_ident(p2)?);
            },
            _ => throw_rules(&[Rule::ref_stmt, Rule::ident], p2)?,
          }
        }
      }
      _ => throw_rules(&[Rule::ref_block, Rule::ref_short], p1)?,
    }
  }

  unreachable!("something went wrong parsing ref_decl")
}

// FIXME: to be fixed
fn parse_ref_stmt(pair: Pair<Rule>) -> ParserResult<RefBlock> {
  let init = RefBlock {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::relation => {
          acc.rel = match Relation::from_str(p1.as_str()) {
            Ok(rel) => rel,
            Err(err) => throw_msg(err, p1)?,
          }
        }
        Rule::ref_ident => {
          let value = parse_ref_ident(p1)?;

          if acc.rel == Relation::Undef {
            acc.lhs = value;
          } else {
            acc.rhs = value;
          }
        }
        Rule::rel_settings => acc.settings = Some(parse_rel_settings(p1)?),
        _ => throw_rules(&[Rule::relation, Rule::ref_ident, Rule::rel_settings], p1)?,
      }

      Ok(acc)
    })
}

fn parse_ref_inline(pair: Pair<Rule>) -> ParserResult<RefInline> {
  let init = RefInline {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::relation => {
          acc.rel = match Relation::from_str(p1.as_str()) {
            Ok(rel) => rel,
            Err(err) => throw_msg(err, p1)?,
          }
        }
        Rule::ref_ident => {
          acc.rhs = parse_ref_ident(p1)?;
        }
        _ => throw_rules(&[Rule::relation, Rule::ref_ident], p1)?,
      }

      Ok(acc)
    })
}

fn parse_ref_ident(pair: Pair<Rule>) -> ParserResult<RefIdent> {
  let mut out = RefIdent {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };
  let mut tmp_tokens = vec![];

  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => tmp_tokens.push(parse_ident(p1)?),
      Rule::ref_composition => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::ident => out.compositions.push(parse_ident(p2)?),
            _ => throw_rules(&[Rule::ident], p2)?,
          }
        }
      }
      _ => throw_rules(&[Rule::ident, Rule::ref_composition], p1)?,
    }
  }

  match tmp_tokens.len() {
    1 => {
      out.table = tmp_tokens.remove(0)
    },
    2 => {
      out.schema = Some(tmp_tokens.remove(0));
      out.table = tmp_tokens.remove(0);
    },
    _ => unreachable!("unwell formatted ident")
  }

  Ok(out)
}

fn parse_table_group_decl(pair: Pair<Rule>) -> ParserResult<TableGroupBlock> {
  let init = TableGroupBlock {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::ident => acc.ident = parse_ident(p1)?,
        Rule::table_group_block => {
          for p2 in p1.into_inner() {
            let mut init = TableGroupItem {
              span_range: s2r(p2.as_span()),
              ..Default::default()
            };
            
            match p2.as_rule() {
              Rule::decl_ident => {
                let (schema, name) = parse_decl_ident(p2)?;
                
                init.schema = schema;
                init.ident_alias = name;

                acc.items.push(init)
              }
              _ => throw_rules(&[Rule::decl_ident], p2)?,
            }
          }
        }
        _ => throw_rules(&[Rule::ident, Rule::table_group_block], p1)?,
      }

      Ok(acc)
    })
}

fn parse_rel_settings(pair: Pair<Rule>) -> ParserResult<RefSettings> {
  let init = RefSettings {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::rel_attribute => {
          for p2 in p1.into_inner() {
            match p2.as_rule() {
              Rule::rel_update => {
                for p3 in p2.into_inner() {
                  acc.on_update = match ReferentialAction::from_str(p3.as_str()) {
                    Ok(val) => Some(val),
                    Err(_) => throw_rules(&[Rule::rel_update], p3)?,
                  }
                }
              }
              Rule::rel_delete => {
                for p3 in p2.into_inner() {
                  acc.on_delete = match ReferentialAction::from_str(p3.as_str()) {
                    Ok(val) => Some(val),
                    Err(_) => throw_rules(&[Rule::rel_delete], p3)?,
                  }
                }
              }
              _ => throw_rules(&[Rule::rel_update, Rule::rel_delete], p2)?,
            }
          }
        }
        _ => throw_rules(&[Rule::rel_attribute], p1)?,
      }

      Ok(acc)
    })
}

fn parse_note_decl(pair: Pair<Rule>) -> ParserResult<NoteBlock> {
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::note_short | Rule::note_block => {
        for p2 in p1.clone().into_inner() {
          match p2.as_rule() {
            Rule::string_value => {
              return parse_string_value(p2.clone()).map(|value| {
                NoteBlock {
                  span_range: s2r(p1.as_span()),
                  value: Literal {
                    span_range: s2r(p2.as_span()),
                    raw: p2.as_str().to_owned(),
                    value: Value::String(value)
                  }
                }
              })
            },
            _ => throw_rules(&[Rule::string_value], p2)?,
          }
        }
      }
      _ => throw_rules(&[Rule::note_short, Rule::note_block], p1)?,
    }
  }

  unreachable!("something went wrong parsing note_decl")
}

fn parse_note_inline(pair: Pair<Rule>) -> ParserResult<String> {
  pair
    .into_inner()
    .try_fold(String::new(), |_, p1| match p1.as_rule() {
      Rule::string_value => parse_string_value(p1),
      _ => throw_rules(&[Rule::string_value], p1)?,
    })
}

fn parse_indexes_decl(pair: Pair<Rule>) -> ParserResult<IndexesBlock> {
  let p1 = pair
    .into_inner()
    .next()
    .ok_or_else(|| unreachable!("something went wrong parsing indexes_decl"))?;

  match p1.as_rule() {
    Rule::indexes_block => parse_indexes_block(p1),
    _ => throw_rules(&[Rule::indexes_block], p1)?,
  }
}

fn parse_indexes_block(pair: Pair<Rule>) -> ParserResult<IndexesBlock> {
  let init = IndexesBlock {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::indexes_single | Rule::indexes_multi => {
          acc.defs.push(parse_indexes_single_multi(p1)?)
        }
        _ => throw_rules(&[Rule::indexes_single, Rule::indexes_multi], p1)?,
      }

      Ok(acc)
    })
}

fn parse_indexes_single_multi(pair: Pair<Rule>) -> ParserResult<IndexesDef> {
  let init = IndexesDef {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::indexes_ident => acc.cols.push(parse_indexes_ident(p1)?),
        Rule::indexes_settings => acc.settings = Some(parse_indexes_settings(p1)?),
        _ => throw_rules(&[Rule::indexes_ident, Rule::indexes_settings], p1)?,
      }

      Ok(acc)
    })
}

fn parse_indexes_ident(pair: Pair<Rule>) -> ParserResult<IndexesColumnType> {
  let p1 = pair
    .into_inner()
    .next()
    .ok_or_else(|| unreachable!("something went wrong at indexes_ident"))?;

  match p1.as_rule() {
    Rule::ident => {
      let value = parse_ident(p1)?;
      Ok(IndexesColumnType::String(value.to_string))
    }
    Rule::backquoted_quoted_string => {
      let p2 = p1
        .into_inner()
        .next()
        .ok_or_else(|| unreachable!("something went wrong at indexes_ident"))?;

      match p2.as_rule() {
        Rule::backquoted_quoted_value => {
          let value = p2.as_str().to_string();
          Ok(IndexesColumnType::Expr(value))
        }
        _ => throw_rules(&[Rule::backquoted_quoted_value], p2)?,
      }
    }
    _ => throw_rules(&[Rule::ident, Rule::backquoted_quoted_string], p1)?,
  }
}

fn parse_indexes_settings(pair: Pair<Rule>) -> ParserResult<IndexesSettings> {
  let init = IndexesSettings {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  pair
    .into_inner()
    .try_fold(init, |mut acc, p1| {
      match p1.as_rule() {
        Rule::indexes_attribute => {
          for p2 in p1.into_inner() {
            match p2.as_rule() {
              Rule::indexes_attribute_key => match p2.as_str() {
                "unique" => acc.is_unique = true,
                "pk" => acc.is_pk = true,
                _ => throw_msg(
                  format!("'{}' key is invalid inside indexes_attribute", p2.as_str()),
                  p2,
                )?,
              },
              Rule::indexes_type => {
                acc.r#type = p2
                  .into_inner()
                  .try_fold(None, |_, p3| match IndexesType::from_str(p3.as_str()) {
                    Ok(val) => Ok(Some(val)),
                    Err(msg) => throw_msg(msg, p3)?,
                  })?
              }
              Rule::indexes_name => p2.into_inner().for_each(|p3| {
                acc.name = Some(p3.into_inner().as_str().to_string());
              }),
              Rule::note_inline => acc.note = Some(parse_note_inline(p2)?),
              _ => throw_rules(
                &[
                  Rule::indexes_attribute_key,
                  Rule::indexes_type,
                  Rule::indexes_name,
                  Rule::note_inline,
                ],
                p2,
              )?,
            }
          }
        }
        _ => throw_rules(&[Rule::indexes_attribute], p1)?,
      }

      Ok(acc)
    })
}

fn parse_string_value(pair: Pair<Rule>) -> ParserResult<String> {
  let mut out = String::new();

  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::triple_quoted_string => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::triple_quoted_value => out = p2.as_str().to_string(),
            _ => throw_rules(&[Rule::triple_quoted_value], p2)?,
          }
        }
      }
      Rule::single_quoted_string => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::single_quoted_value => out = p2.as_str().to_string(),
            _ => throw_rules(&[Rule::single_quoted_value], p2)?,
          }
        }
      }
      _ => throw_rules(
        &[Rule::triple_quoted_string, Rule::single_quoted_string],
        p1,
      )?,
    }
  }

  Ok(out)
}

fn parse_value(pair: Pair<Rule>) -> ParserResult<Value> {
  let p1 = pair
    .into_inner()
    .next()
    .ok_or_else(|| unreachable!("something went wrong at value"))?;

  match p1.as_rule() {
    Rule::string_value => {
      let value = parse_string_value(p1)?;

      Ok(Value::String(value))
    }
    Rule::number_value => {
      let p2 = p1
        .into_inner()
        .next()
        .ok_or_else(|| unreachable!("something went wrong at value"))?;

      match p2.as_rule() {
        Rule::decimal => match p2.as_str().parse::<f64>() {
          Ok(val) => Ok(Value::Decimal(val)),
          Err(err) => throw_msg(err.to_string(), p2)?,
        },
        Rule::integer => match p2.as_str().parse::<i64>() {
          Ok(val) => Ok(Value::Integer(val)),
          Err(err) => throw_msg(err.to_string(), p2)?,
        },
        _ => throw_rules(&[Rule::decimal, Rule::integer], p2)?,
      }
    }
    Rule::boolean_value => {
      if let Ok(v) = Value::from_str(p1.as_str()) {
        Ok(v)
      } else {
        throw_msg(
          format!("'{}' is incompatible with boolean value", p1.as_str()),
          p1,
        )?
      }
    }
    Rule::hex_value => Ok(Value::HexColor(p1.as_str().to_string())),
    Rule::backquoted_quoted_string => Ok(Value::Expr(p1.into_inner().as_str().to_string())),
    _ => throw_rules(
      &[
        Rule::string_value,
        Rule::number_value,
        Rule::boolean_value,
        Rule::hex_value,
        Rule::backquoted_quoted_string,
      ],
      p1,
    )?,
  }
}

fn parse_decl_ident(pair: Pair<Rule>) -> ParserResult<(Option<Ident>, Ident)> {
  let mut tmp_tokens = vec![];

  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => tmp_tokens.push(parse_ident(p1)?),
      _ => throw_rules(&[Rule::ident], p1)?,
    }
  }

  let (schema, name) = match tmp_tokens.len() {
    1 => {
      (None, tmp_tokens.remove(0))
    },
    2 => {
      let schema = Some(tmp_tokens.remove(0));

      (schema, tmp_tokens.remove(0))
    }
    _ => unreachable!("unwell formatted decl_ident")
  };

  Ok((schema, name))
}

fn parse_ident(pair: Pair<Rule>) -> ParserResult<Ident> {
  let p1 = pair
    .into_inner()
    .next()
    .ok_or_else(|| unreachable!("something went wrong at ident"))?;

  match p1.as_rule() {
    Rule::var => Ok(Ident {
      span_range: s2r(p1.as_span()),
      raw: p1.as_str().to_string(),
      to_string: p1.as_str().to_string()
    }),
    Rule::double_quoted_string => Ok(Ident {
      span_range: s2r(p1.as_span()),
      raw: p1.as_str().to_string(),
      to_string: p1.into_inner().as_str().to_string()
    }),
    _ => throw_rules(&[Rule::var, Rule::double_quoted_string], p1)?,
  }
}

pub fn parse_attribute(pair: Pair<Rule>) -> ParserResult<Attribute> {
  let mut init = Attribute {
    span_range: s2r(pair.as_span()),
    ..Default::default()
  };

  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::spaced_var => {
        init.key = Ident {
          span_range: s2r(p1.as_span()),
          raw: p1.as_str().to_owned(),
          to_string: p1.as_str().to_owned()
        };
      },
      Rule::value =>{
        init.value = Some(Literal {
          span_range: s2r(p1.as_span()),
          raw: p1.as_str().to_owned(),
          value: parse_value(p1)?
        })
      },
      _ => throw_rules(&[Rule::var, Rule::value], p1)?
    }
  }

  Ok(init)
}

pub fn parse_property(pair: Pair<Rule>) -> ParserResult<Property> {
  let init = parse_attribute(pair.clone())?;

  match init.value {
    Some(value) => {
      Ok(Property {
        span_range: init.span_range,
        key: init.key,
        value
      })
    }
    None => throw_rules(&[Rule::property], pair)
  }
}