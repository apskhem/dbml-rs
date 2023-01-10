use std::ops::Range;

use crate::parser::Rule;

use pest::Span;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub type AnalyzingResult<T> = Result<T, Error<Rule>>;

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
  UnsupportedSyntax
}

pub fn throw_msg<T>(msg: impl ToString, pair: Pair<Rule>) -> AnalyzingResult<T> {
  Err(
    Error::new_from_span(
      ErrorVariant::CustomError { message: msg.to_string() },
      pair.as_span()
    )
  )
}

pub fn throw_err<T>(err: Err, span_range: Range<usize>, input: &str) -> AnalyzingResult<T> {
  let span = Span::new(input, span_range.start, span_range.end).unwrap();

  let msg = match err {
    Err::ProjectSettingNotFound => "the project block is required",
    _ => "unlisted error"
  };
  
  Err(
    Error::new_from_span(
      ErrorVariant::CustomError { message: msg.to_string() },
      span
    )
  )
}
