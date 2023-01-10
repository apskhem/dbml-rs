mod err;

use std::collections::HashMap;
use std::str::FromStr;

use err::*;

use pest::Parser;
use pest::iterators::Pair;

use crate::ast::enums::*;
use crate::ast::indexes::*;
use crate::ast::project;
use crate::ast::project::*;
use crate::ast::refs::*;
use crate::ast::table::*;
use crate::ast::table_group::*;
use crate::ast::schema::*;
use crate::utils::s2r;

#[derive(Parser)]
#[grammar = "src/dbml.pest"]
struct DBMLParser;

pub fn parse(input: &str) -> ParserResult<SchemaBlock> {
  let pairs = DBMLParser::parse(Rule::schema, input)?;

  for pair in pairs {
    match pair.as_rule() {
      Rule::schema => {
        return Ok(parse_schema(pair, input)?);
      },
      _ => throw_rules(&[Rule::schema], pair)?
    }
  }

  unreachable!("unhandled parsing error!");
}

fn parse_schema<'a>(pair: Pair<Rule>, input: &'a str) -> ParserResult<SchemaBlock<'a>> {
  let init = SchemaBlock::new(&input, s2r(pair.as_span()));

  pair.into_inner().try_fold(init, |mut acc, p1| {
    match p1.as_rule() {
      Rule::project_decl => acc.project = Some(parse_project_decl(p1)?),
      Rule::table_decl => acc.tables.push(parse_table_decl(p1)?),
      Rule::enum_decl => acc.enums.push(parse_enum_decl(p1)?),
      Rule::ref_decl => acc.refs.push(parse_ref_decl(p1)?),
      Rule::table_group_decl => acc.table_groups.push(parse_table_group_decl(p1)?),
      Rule::EOI => (),
      _ => throw_rules(&[Rule::project_decl, Rule::table_decl, Rule::enum_decl, Rule::ref_decl, Rule::table_group_decl], p1)?,
    };

    Ok(acc)
  })
}

fn parse_project_decl(pair: Pair<Rule>) -> ParserResult<ProjectBlock> {
  let init = ProjectBlock::from(pair.as_span());

  pair.into_inner().try_fold(init, |mut acc, p1| {
    match p1.as_rule() {
      Rule::ident => {
        acc.name = parse_ident(p1)?
      },
      Rule::project_block => {
        p1.into_inner().try_for_each(|p2| {
          match p2.as_rule() {
            Rule::project_stmt => {
              let (key, value) = parse_project_stmt(p2.clone())?;

              match key.as_str() {
                "database_type" => {
                  acc.database_type = match project::DatabaseType::from_str(&value) {
                    Ok(val) => val,
                    Err(msg) => throw_msg(msg, p2)?
                  }
                },
                _ => throw_msg(format!("'{}' key is invalid inside project_block", key), p2)?,
              }
            },
            Rule::note_decl => {
              acc.note = Some(parse_note_decl(p2)?)
            },
            _ => throw_rules(&[Rule::project_stmt, Rule::note_decl], p2)?,
          };

          Ok(())
        })?
      },
      _ => throw_rules(&[Rule::project_block], p1)?,
    }

    Ok(acc)
  })
}

fn parse_project_stmt(pair: Pair<Rule>) -> ParserResult<(String, String)> {
  pair.into_inner().try_fold((String::new(), String::new()), |mut acc, p1| {
    match p1.as_rule() {
      Rule::var => acc.0 = p1.as_str().to_string(),
      Rule::string_value => acc.1 = parse_string_value(p1)?,
      _ => throw_rules(&[Rule::var, Rule::string_value], p1)?,
    }
    
    Ok(acc)
  })
}

fn parse_table_decl(pair: Pair<Rule>) -> ParserResult<TableBlock> {
  pair.into_inner().try_fold(TableBlock::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::decl_ident => {
        let (schema, name) = parse_decl_ident(p1)?;
        
        acc.ident.name = name;
        acc.ident.schema = schema;
      },
      Rule::table_alias => {
        acc.ident.alias = Some(p1.into_inner().as_str().to_string())
      },
      Rule::table_block => {
        p1.into_inner().try_for_each(|p2| {
          match p2.as_rule() {
            Rule::table_col => {
              acc.cols.push(parse_table_col(p2)?)
            },
            Rule::note_decl => {
              acc.note = Some(parse_note_decl(p2)?)
            },
            Rule::indexes_decl => {
              acc.indexes = Some(parse_indexes_decl(p2)?)
            },
            _ => throw_rules(&[Rule::table_col, Rule::note_decl, Rule::indexes_decl], p2)?,
          }

          Ok(())
        })?
      },
      Rule::table_settings => {
        acc.settings = Some(parse_table_settings(p1)?);
      },
      _ => throw_rules(&[Rule::decl_ident, Rule::table_alias, Rule::table_block, Rule::table_settings], p1)?,
    }

    Ok(acc)
  })
}

fn parse_table_settings(pair: Pair<Rule>) -> ParserResult<HashMap<String, Value>> {
  pair.into_inner().try_fold(HashMap::new(), |mut acc, p2| {
    match p2.as_rule() {
      Rule::table_attribute => {
        let p2_cloned = p2.clone();
        let mut s_key = None;
        let mut s_val = None;
        
        for p3 in p2.into_inner() {
          if s_key.is_none() {
            s_key = Some(p3.as_str().to_string());
          }
          else if s_val.is_none() {
            s_val = Some(parse_value(p3)?);
          }
          else {
            throw_rules(&[Rule::table_attribute], p3)?
          }
        }
        
        let (s_key, s_val) = match (s_key, s_val) {
          (Some(k), Some(v)) => (k, v),
          _ => throw_rules(&[Rule::table_attribute], p2_cloned)?
        };

        acc.insert(s_key, s_val);
      },
      _ => throw_rules(&[Rule::table_attribute], p2)?,
    }

    Ok(acc)
  })
}

fn parse_table_col(pair: Pair<Rule>) -> ParserResult<TableColumn> {
  pair.into_inner().try_fold(TableColumn::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::ident => {
        acc.name = parse_ident(p1)?
      },
      Rule::col_type => {
        acc.r#type = parse_col_type(p1)?;
      },
      Rule::col_settings => {
        acc.settings = parse_col_settings(p1)?
      },
      _ => throw_rules(&[Rule::ident, Rule::col_type, Rule::col_settings], p1)?,
    }

    Ok(acc)
  })
}

fn parse_col_type(pair: Pair<Rule>) -> ParserResult<ColumnType> {
  let mut out = ColumnType::default();

  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::col_type_quoted | Rule::col_type_unquoted => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::var => {
              out.type_name = ColumnTypeName::Raw(p2.as_str().to_string())
            },
            Rule::double_quoted_value => {
              out.type_name = ColumnTypeName::Raw(p2.as_str().to_string())
            },
            Rule::col_type_arg => {
              out.args = parse_col_type_arg(p2)?
            },
            Rule::col_type_array => {
              let val = p2.into_inner().try_fold(None, |_, p3| {
                match p3.as_rule() {
                  Rule::integer => {
                    let val = match p3.as_str().parse::<usize>() {
                      Ok(val) => Some(val),
                      _ => throw_msg("cannot parse the value into usize", p3)?
                    };

                    Ok(val)
                  },
                  _ => throw_rules(&[Rule::integer], p3)?
                }
              })?;

              out.arrays.push(val)
            },
            _ => throw_rules(&[Rule::var, Rule::col_type_arg], p2)?,
          }
        }
      },
      _ => throw_rules(&[Rule::col_type_quoted, Rule::col_type_unquoted], p1)?,
    }
  }

  Ok(out)
}

fn parse_col_type_arg(pair: Pair<Rule>) -> ParserResult<Vec<Value>> {
  pair.into_inner().try_fold(vec![], |mut acc, p1| {
    match p1.as_rule() {
      Rule::value => {
        acc.push(parse_value(p1)?)
      },
      _ => throw_rules(&[Rule::value], p1)?,
    }

    Ok(acc)
  })
}

fn parse_col_settings(pair: Pair<Rule>) -> ParserResult<ColumnSettings> {
  pair.into_inner().try_fold(ColumnSettings::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::col_attribute => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::col_attribute_key => {
              match p2.as_str() {
                "unique" => acc.is_unique = true,
                "primary key" | "pk" => acc.is_pk = true,
                "null" => acc.is_nullable = true,
                "not null" => (),
                "increment" => acc.is_incremental = true,
                _ => throw_msg(format!("'{}' is invalid col_attribute_key!", p2.as_str()), p2)?,
              }
            }
            Rule::col_default => {
              for p3 in p2.into_inner() {
                match p3.as_rule() {
                  Rule::value => acc.default = Some(parse_value(p3)?),
                  _ => throw_rules(&[Rule::value], p3)?,
                }
              }
            },
            Rule::note_inline => {
              acc.note = Some(parse_note_inline(p2)?)
            },
            Rule::ref_inline => {
              acc.refs.push(parse_ref_inline(p2)?)
            },
            _ => throw_rules(&[Rule::col_attribute_key, Rule::col_default, Rule::note_inline, Rule::ref_inline], p2)?,
          }
        }
      },
      _ => throw_rules(&[Rule::col_attribute], p1)?,
    }

    Ok(acc)
  })
}

fn parse_enum_decl(pair: Pair<Rule>) -> ParserResult<EnumBlock> {
  pair.into_inner().try_fold(EnumBlock::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::decl_ident => {
        let (schema, name) = parse_decl_ident(p1)?;
        
        acc.ident.name = name;
        acc.ident.schema = schema;
      },
      Rule::enum_block => {
        acc.values = parse_enum_block(p1)?
      },
      _ => throw_rules(&[Rule::decl_ident, Rule::enum_block], p1)?,
    }

    Ok(acc)
  })
}

fn parse_enum_block(pair: Pair<Rule>) -> ParserResult<Vec<EnumValue>> {
  pair.into_inner().try_fold(vec![], |mut acc, p1| {
    match p1.as_rule() {
      Rule::enum_value => {
        acc.push(parse_enum_value(p1)?)
      },
      _ => throw_rules(&[Rule::enum_value], p1)?,
    }

    Ok(acc)
  })
}

fn parse_enum_value(pair: Pair<Rule>) -> ParserResult<EnumValue> {
  pair.into_inner().try_fold(EnumValue::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::ident => {
        acc.value = parse_ident(p1)?
      },
      Rule::enum_settings => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::enum_attribute => {
              for p3 in p2.into_inner() {
                match p3.as_rule() {
                  Rule::note_inline => {
                    acc.note = Some(parse_note_inline(p3)?)
                  },
                  _ => throw_rules(&[Rule::note_inline], p3)?,
                }
              }
            },
            _ => throw_rules(&[Rule::enum_attribute], p2)?,
          }
        }
      },
      _ => throw_rules(&[Rule::ident, Rule::enum_settings], p1)?,
    }

    Ok(acc)
  })
}

fn parse_ref_decl(pair: Pair<Rule>) -> ParserResult<RefBlock> {
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ref_block | Rule::ref_short => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::ref_stmt => {
              return parse_ref_stmt(p2)
            },
            _ => throw_rules(&[Rule::ref_stmt], p2)?,
          }
        }
      },
      _ => throw_rules(&[Rule::ref_block, Rule::ref_short], p1)?,
    }
  }

  unreachable!("something went wrong parsing ref_decl!")
}

// FIXME: to be fixed
fn parse_ref_stmt(pair: Pair<Rule>) -> ParserResult<RefBlock> {
  pair.into_inner().try_fold(RefBlock::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::relation => {
        acc.rel = match Relation::from_str(p1.as_str()) {
          Ok(rel) => rel,
          Err(_) => throw_msg(format!("'{:?}' type is not supported!", p1.as_str()), p1)?
        }
      },
      Rule::ref_ident => {
        let value = parse_ref_ident(p1)?;

        if acc.rel == Relation::Undef {
          acc.lhs = value;
        } else {
          acc.rhs = value;
        }
      },
      Rule::rel_settings => {
        acc.settings = Some(parse_rel_settings(p1)?)
      },
      _ => throw_rules(&[Rule::relation, Rule::ref_ident, Rule::rel_settings], p1)?,
    }

    Ok(acc)
  })
}

fn parse_ref_inline(pair: Pair<Rule>) -> ParserResult<RefInline> {
  pair.into_inner().try_fold(RefInline::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::relation => {
        acc.rel = match Relation::from_str(p1.as_str()) {
          Ok(rel) => rel,
          Err(_) => throw_msg(format!("'{:?}' type is not supported!", p1.as_str()), p1)?
        }
      },
      Rule::ref_ident => {
        acc.rhs = parse_ref_ident(p1)?;
      },
      _ => throw_rules(&[Rule::relation, Rule::ref_ident], p1)?,
    }

    Ok(acc)
  })
}

fn parse_ref_ident(pair: Pair<Rule>) -> ParserResult<RefIdent> {
  let mut out = RefIdent::default();
  let mut tmp_tokens = vec![];
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => {
        tmp_tokens.push(parse_ident(p1)?)
      },
      Rule::ref_composition => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::ident => {
              out.compositions.push(parse_ident(p2)?)
            },
            _ => throw_rules(&[Rule::ident], p2)?,
          }
        }
      },
      _ => throw_rules(&[Rule::ident, Rule::ref_composition], p1)?,
    }
  }

  if tmp_tokens.len() == 2 {
    out.schema = Some(tmp_tokens.remove(0));
    out.table = tmp_tokens.remove(0);
  } else if tmp_tokens.len() == 1 {
    out.table = tmp_tokens.remove(0);
  } else {
    unreachable!("unwell formatted ident!");
  }

  Ok(out)
}

fn parse_table_group_decl(pair: Pair<Rule>) -> ParserResult<TableGroupBlock> {
  pair.into_inner().try_fold(TableGroupBlock::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::ident => {
        acc.name = parse_ident(p1)?
      },
      Rule::table_group_block => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::decl_ident => {
              let (schema, name) = parse_decl_ident(p2)?;
              
              let value = TableGroupIdent {
                schema,
                ident_alias: name,
              };

              acc.table_idents.push(value)
            },
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
  pair.into_inner().try_fold(RefSettings::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::rel_attribute => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::rel_update  => {
              for p3 in p2.into_inner() {
                acc.on_update = match ReferentialAction::from_str(p3.as_str()) {
                  Ok(val) => Some(val),
                  Err(_) => throw_rules(&[Rule::rel_update], p3)?,
                }
              }
            },
            Rule::rel_delete  => {
              for p3 in p2.into_inner() {
                acc.on_delete = match ReferentialAction::from_str(p3.as_str()) {
                  Ok(val) => Some(val),
                  Err(_) => throw_rules(&[Rule::rel_delete], p3)?,
                }
              }
            },
            _ => throw_rules(&[Rule::rel_update, Rule::rel_delete], p2)?,
          }
        }
      },
      _ => throw_rules(&[Rule::rel_attribute], p1)?,
    }

    Ok(acc)
  })
}

fn parse_note_decl(pair: Pair<Rule>) -> ParserResult<String> {
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::note_short | Rule::note_block => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::string_value => {
              return parse_string_value(p2)
            },
            _ => throw_rules(&[Rule::string_value], p2)?,
          }
        }
      },
      _ => throw_rules(&[Rule::note_short, Rule::note_block], p1)?,
    }
  }

  unreachable!("something went wrong parsing note_decl!")
}

fn parse_note_inline(pair: Pair<Rule>) -> ParserResult<String> {
  pair.into_inner().try_fold(String::new(), |_, p1| {
    match p1.as_rule() {
      Rule::string_value => {
        parse_string_value(p1)
      },
      _ => throw_rules(&[Rule::string_value], p1)?,
    }
  })
}

fn parse_indexes_decl(pair: Pair<Rule>) -> ParserResult<IndexesBlock> {
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::indexes_block => {
        return parse_indexes_block(p1)
      },
      _ => throw_rules(&[Rule::indexes_block], p1)?,
    }
  }

  unreachable!("something went wrong parsing indexes_decl!")
}

fn parse_indexes_block(pair: Pair<Rule>) -> ParserResult<IndexesBlock> {
  pair.into_inner().try_fold(IndexesBlock::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::indexes_single | Rule::indexes_multi => {
        acc.defs.push(parse_indexes_single_multi(p1)?)
      },
      _ => throw_rules(&[Rule::indexes_single, Rule::indexes_multi], p1)?,
    }

    Ok(acc)
  })
}

fn parse_indexes_single_multi(pair: Pair<Rule>) -> ParserResult<IndexesDef> {
  pair.into_inner().try_fold(IndexesDef::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::indexes_ident => {
        acc.idents.push(parse_indexes_ident(p1)?)
      },
      Rule::indexes_settings => {
        acc.settings = Some(parse_indexes_settings(p1)?)
      },
      _ => throw_rules(&[Rule::indexes_ident, Rule::indexes_settings], p1)?,
    }

    Ok(acc)
  })
}

fn parse_indexes_ident(pair: Pair<Rule>) -> ParserResult<IndexesIdent> {
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => {
        let value = parse_ident(p1)?;
        return Ok(IndexesIdent::String(value))
      },
      Rule::backquoted_quoted_string => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::backquoted_quoted_value => {
              let value = p2.as_str().to_string();
              return Ok(IndexesIdent::Expr(value))
            },
            _ => throw_rules(&[Rule::backquoted_quoted_value], p2)?,
          }
        }
      },
      _ => throw_rules(&[Rule::ident, Rule::backquoted_quoted_string], p1)?,
    }
  }

  unreachable!("something went wrong at indexes_ident");
}

fn parse_indexes_settings(pair: Pair<Rule>) -> ParserResult<IndexesSettings> {
  pair.into_inner().try_fold(IndexesSettings::default(), |mut acc, p1| {
    match p1.as_rule() {
      Rule::indexes_attribute => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::indexes_attribute_key => {
              match p2.as_str() {
                "unique" => acc.is_unique = true,
                "pk" => acc.is_pk = true,
                _ => throw_msg(format!("'{}' key is invalid inside indexes_attribute", p2.as_str()), p2)?,
              }
            },
            Rule::indexes_type => {
              acc.r#type = p2
                .into_inner()
                .try_fold(None, |_, p3| {
                  match IndexesType::from_str(p3.as_str()) {
                    Ok(val) => Ok(Some(val)),
                    Err(msg) => throw_msg(msg, p3)?
                  }
                })?
            },
            Rule::indexes_name => {
              p2.into_inner().for_each(|p3| {
                acc.name = Some(p3.into_inner().as_str().to_string());
              })
            },
            Rule::note_inline => {
              acc.note = Some(parse_note_inline(p2)?)
            },
            _ => throw_rules(&[Rule::indexes_attribute_key, Rule::indexes_type, Rule::indexes_name, Rule::note_inline], p2)?,
          }
        }
      },
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
            Rule::triple_quoted_value => {
              out = p2.as_str().to_string()
            },
            _ => throw_rules(&[Rule::triple_quoted_value], p2)?,
          }
        }
      },
      Rule::single_quoted_string => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::single_quoted_value => {
              out = p2.as_str().to_string()
            },
            _ => throw_rules(&[Rule::single_quoted_value], p2)?,
          }
        }
      },
      _ => throw_rules(&[Rule::triple_quoted_string, Rule::single_quoted_string], p1)?,
    }
  }

  Ok(out)
}

fn parse_value(pair: Pair<Rule>) -> ParserResult<Value> {
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::string_value => {
        let value = parse_string_value(p1)?;

        return Ok(Value::String(value));
      },
      Rule::number_value => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::decimal => {
              return match p2.as_str().parse::<f32>() {
                Ok(val) => Ok(Value::Decimal(val)),
                Err(err) => throw_msg(err.to_string(), p2)?,
              };
            },
            Rule::integer => {
              return match p2.as_str().parse::<i32>() {
                Ok(val) => Ok(Value::Integer(val)),
                Err(err) => throw_msg(err.to_string(), p2)?,
              };
            },
            _ => throw_rules(&[Rule::decimal, Rule::integer], p2)?,
          }
        }
      },
      Rule::boolean_value => {
        for p2 in p1.into_inner() {
          return match p2.as_str() {
            "true" => Ok(Value::Bool(true)),
            "false" => Ok(Value::Bool(false)),
            "null" => Ok(Value::Null),
            _ => throw_msg(format!("'{}' is incompatible with boolean value", p2.as_str()), p2)?,
          }
        }
      },
      Rule::hex_value => {
        return Ok(Value::HexColor(p1.as_str().to_string()))
      },
      Rule::backquoted_quoted_string => {
        return Ok(Value::Expr(p1.into_inner().as_str().to_string()))
      },
      _ => throw_rules(&[
        Rule::string_value,
        Rule::number_value,
        Rule::boolean_value,
        Rule::hex_value,
        Rule::backquoted_quoted_string
      ], p1)?,
    }
  }

  unreachable!("something went wrong at value!")
}

fn parse_decl_ident(pair: Pair<Rule>) -> ParserResult<(Option<String>, String)> {
  let mut tmp_tokens = vec![];

  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => tmp_tokens.push(parse_ident(p1)?),
      _ => throw_rules(&[Rule::ident], p1)?,
    }
  }

  let (schema, name) = if tmp_tokens.len() == 2 {
    let schema = Some(tmp_tokens.remove(0));

    (schema, tmp_tokens.remove(0))
  } else if tmp_tokens.len() == 1 {
    (None, tmp_tokens.remove(0))
  } else {
    unreachable!("unwell formatted decl_ident!")
  };

  Ok((schema, name))
}

fn parse_ident(pair: Pair<Rule>) -> ParserResult<String> {
  for p1 in pair.into_inner() {
    return match p1.as_rule() {
      Rule::var => {
        Ok(p1.as_str().to_string())
      },
      Rule::double_quoted_string => {
        Ok(p1.into_inner().as_str().to_string())
      },
      _ => throw_rules(&[Rule::var, Rule::double_quoted_string], p1)?,
    }
  }

  unreachable!("something went wrong at ident!")
}
