use core::fmt;
use std::ops::Range;

use pest::error::{
  Error,
  ErrorVariant,
};
use pest::Span;

use crate::parser::Rule;

pub type AnalyzerResult<T> = Result<T, Error<Rule>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Err {
  NullablePrimaryKey,
  ArrayPrimaryKey,
  InvalidIndexesSetting,
  DuplicatedProjectSetting,
  DuplicatedPrimaryKey,
  DuplicatedUniqueKey,
  DuplicatedIndexKey,
  DuplicatedTableName,
  DuplicatedColumnName,
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

impl fmt::Display for Err {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Err::NullablePrimaryKey => write!(f, "Nullable primary key is not allowed"),
      Err::ArrayPrimaryKey => write!(f, "Array primary key is not allowed"),
      Err::InvalidIndexesSetting => write!(f, "Invalid indexes definition setting: can specify either pk, unique, or type within a setting"),
      Err::DuplicatedProjectSetting => write!(f, "Duplicated project setting"),
      Err::DuplicatedPrimaryKey => write!(f, "Duplicated primary key"),
      Err::DuplicatedUniqueKey => write!(f, "Duplicated unique key"),
      Err::DuplicatedIndexKey => write!(f, "Duplicated index key"),
      Err::DuplicatedTableName => write!(f, "Duplicated table name"),
      Err::DuplicatedColumnName => write!(f, "Duplicated column name"),
      Err::DuplicatedRelation => write!(f, "Duplicated relation"),
      Err::DuplicatedEnumName => write!(f, "Duplicated enum name"),
      Err::DuplicatedEnumValue => write!(f, "Duplicated enum value"),
      Err::DuplicatedTableGroupName => write!(f, "Duplicated table group name"),
      Err::ProjectSettingNotFound => write!(f, "Project setting not found"),
      Err::TableGroupNotFound => write!(f, "Table group not found"),
      Err::SchemaNotFound => write!(f, "Schema not found"),
      Err::TableNotFound => write!(f, "Table not found"),
      Err::ColumnNotFound => write!(f, "Column not found"),
      Err::EnumNotFound => write!(f, "Enum not found"),
      Err::EnumValueNotFound => write!(f, "Enum value not found"),
      Err::MismatchedForeignKeyType => write!(f, "Mismatched foreign key type"),
      Err::MismatchedCompositeForeignKey => write!(f, "Mismatched composite foreign key"),
      Err::UnsupportedSyntax => write!(f, "Unsupported syntax"),
    }
  }
}

pub(super) fn throw_err<T>(err: Err, span_range: Range<usize>, input: &str) -> AnalyzerResult<T> {
  let span = Span::new(input, span_range.start, span_range.end).unwrap();

  Err(Error::new_from_span(
    ErrorVariant::CustomError {
      message: err.to_string(),
    },
    span,
  ))
}
