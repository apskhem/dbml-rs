use std::ops::Range;

use derive_more::Display;

use pest::error::{
  Error,
  ErrorVariant,
};
use pest::Span;

use crate::parser::Rule;

pub type AnalyzerResult<T> = Result<T, Error<Rule>>;

#[derive(Debug, PartialEq, Eq, Clone, Display)]
pub enum Err {
  #[display(fmt = "Nullable primary key is not allowed")]
  NullablePrimaryKey,
  #[display(fmt = "Array primary key is not allowed")]
  ArrayPrimaryKey,
  #[display(fmt = "Invalid indexes definition setting: can specify either 'pk', 'unique', or 'type' within a setting")]
  InvalidIndexesSetting,
  #[display(fmt = "Duplicated attribute key")]
  DuplicatedAttributeKey,
  #[display(fmt = "Duplicated property key")]
  DuplicatedPropertyKey,
  #[display(fmt = "Duplicated project setting")]
  DuplicatedProjectSetting,
  #[display(fmt = "Duplicated primary key")]
  DuplicatedPrimaryKey,
  #[display(fmt = "Duplicated unique key")]
  DuplicatedUniqueKey,
  #[display(fmt = "Duplicated index key")]
  DuplicatedIndexKey,
  #[display(fmt = "Duplicated table name")]
  DuplicatedTableName,
  #[display(fmt = "Duplicated column name")]
  DuplicatedColumnName,
  #[display(fmt = "Duplicated relation")]
  DuplicatedRelation,
  #[display(fmt = "Duplicated enum name")]
  DuplicatedEnumName,
  #[display(fmt = "Duplicated enum value")]
  DuplicatedEnumValue,
  #[display(fmt = "Duplicated table group name")]
  DuplicatedTableGroupName,
  #[display(fmt = "Duplicated table group item")]
  DuplicatedTableGroupItem,
  #[display(fmt = "Duplicated alias")]
  DuplicatedAlias,
  #[display(fmt = "Conflicted nullable setting: 'null' and 'not null' must not appear within the same setting")]
  ConflictedNullableSetting,
  #[display(fmt = "Project setting not found")]
  ProjectSettingNotFound,
  #[display(fmt = "Invalid enum")]
  InvalidEnum,
  #[display(fmt = "Invalid data type")]
  InvalidDataType,
  #[display(fmt = "Invalid data type arguments: '{}' requires only {} argument(s)", raw_type, n_arg)]
  InvalidDataTypeArguments { raw_type: String, n_arg: u32 },
  #[display(fmt = "Invalid argument value: the value for the argument is not integer")]
  InvalidArgumentValue,
  #[display(fmt = "Invalid default value: the defualt value '{}' is not associated with '{}'", raw_value, raw_type)]
  InvalidDefaultValue { raw_value: String, raw_type: String },
  #[display(fmt = "Default null in non-nullable: the default value cannot be null in non-nullable field")]
  DefaultNullInNonNullable,
  #[display(fmt = "Data type exceeded: the defualt value exceeds the maximum value of '{}'", raw_type)]
  DataTypeExceeded { raw_type: String },
  #[display(fmt = "Table group not found")]
  TableGroupNotFound,
  #[display(fmt = "Schema not found")]
  SchemaNotFound,
  #[display(fmt = "Table not found")]
  TableNotFound,
  #[display(fmt = "Column not found")]
  ColumnNotFound,
  #[display(fmt = "Enum not found")]
  EnumNotFound,
  #[display(fmt = "Enum value not found")]
  EnumValueNotFound,
  #[display(fmt = "Mismatched foreign key type")]
  MismatchedForeignKeyType,
  #[display(fmt = "Mismatched composite foreign key")]
  MismatchedCompositeForeignKey,
  #[display(fmt = "Unsupported syntax")]
  UnsupportedSyntax,
}

pub(super) fn throw_err<T>(err: Err, span_range: &Range<usize>, input: &str) -> AnalyzerResult<T> {
  let span = Span::new(input, span_range.start, span_range.end).unwrap();

  Err(Error::new_from_span(
    ErrorVariant::CustomError {
      message: err.to_string(),
    },
    span,
  ))
}
