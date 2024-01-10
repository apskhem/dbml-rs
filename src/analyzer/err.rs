use std::ops::Range;

use pest::error::{
  Error,
  ErrorVariant,
};
use pest::iterators::Pair;
use pest::Span;

use crate::parser::Rule;

pub type AnalyzerResult<T> = Result<T, Error<Rule>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Err {
  NullablePrimaryKey,
  ArrayPrimaryKey,
  ConflictedPrimaryKey,
  DuplicatedProjectSetting,
  DuplicatedPrimaryKey,
  DuplicatedTableName,
  DuplicatedRelation,
  DuplicatedEnumName,
  DuplicatedEnumValue,
  DuplicatedTableGroupName,
  ProjectSettingNotFound,
  TableGroupNotFound,
  SchemaNotFound,
  TableNotFound,
  ColumnNotFound,
  EnumNotFound,
  EnumValueNotFound,
  MismatchedForeignKeyType,
  MismatchedCompositeForeignKey,
  UnsupportedSyntax,
}

pub fn throw_msg<T>(msg: impl ToString, pair: Pair<Rule>) -> AnalyzerResult<T> {
  Err(Error::new_from_span(
    ErrorVariant::CustomError {
      message: msg.to_string(),
    },
    pair.as_span(),
  ))
}

pub fn throw_err<T>(err: Err, span_range: Range<usize>, input: &str) -> AnalyzerResult<T> {
  let span = Span::new(input, span_range.start, span_range.end).unwrap();

  let msg = match err {
    Err::ProjectSettingNotFound => "the project block is required",
    _ => "error unexpected",
  };

  Err(Error::new_from_span(
    ErrorVariant::CustomError {
      message: msg.to_string(),
    },
    span,
  ))
}
