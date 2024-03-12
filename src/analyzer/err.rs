use alloc::string::{
  String,
  ToString,
};
use core::ops::Range;

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
  #[display(fmt = "Nullable primary key: a nullable primary is not allowed")]
  NullablePrimaryKey,
  #[display(fmt = "Array primary key: an array with primary is not allowed")]
  ArrayPrimaryKey,
  #[display(fmt = "Invalid indexes setting: can specify either 'pk', 'unique', or 'type' within a setting")]
  InvalidIndexesSetting,
  #[display(fmt = "Duplicate attribute key")]
  DuplicateAttributeKey,
  #[display(fmt = "Duplicate property key")]
  DuplicatePropertyKey,
  #[display(fmt = "Duplicate project setting")]
  DuplicateProjectSetting,
  #[display(fmt = "Duplicate primary key")]
  DuplicatePrimaryKey,
  #[display(fmt = "Duplicate unique key")]
  DuplicateUniqueKey,
  #[display(fmt = "Duplicate index key")]
  DuplicateIndexKey,
  #[display(fmt = "Duplicate table name")]
  DuplicateTableName,
  #[display(fmt = "Duplicate column name")]
  DuplicateColumnName,
  #[display(fmt = "Conflict relation")]
  ConflictRelation,
  #[display(fmt = "Duplicate enum name")]
  DuplicateEnumName,
  #[display(fmt = "Duplicate enum value")]
  DuplicateEnumValue,
  #[display(fmt = "Duplicate table group name")]
  DuplicateTableGroupName,
  #[display(fmt = "Duplicate table group item")]
  DuplicateTableGroupItem,
  #[display(fmt = "Duplicate alias")]
  DuplicateAlias,
  #[display(fmt = "Conflict nullable setting: 'null' and 'not null' must not appear within the same setting")]
  ConflictNullableSetting,
  #[display(fmt = "Project setting not found")]
  ProjectSettingNotFound,
  #[display(fmt = "Empty indexes block")]
  EmptyIndexesBlock,
  #[display(fmt = "Invalid enum")]
  InvalidEnum,
  #[display(fmt = "Invalid data type")]
  InvalidDataType,
  #[display(
    fmt = "Invalid data type arguments: '{}' requires only {} argument(s)",
    raw_type,
    n_arg
  )]
  InvalidDataTypeArguments { raw_type: String, n_arg: u32 },
  #[display(fmt = "Invalid argument value: the value for the argument is not integer")]
  InvalidArgumentValue,
  #[display(
    fmt = "Invalid default value: the defualt value '{}' is not associated with '{}'",
    raw_value,
    raw_type
  )]
  InvalidDefaultValue { raw_value: String, raw_type: String },
  #[display(fmt = "Default null in non-nullable: the default value cannot be null in non-nullable field")]
  DefaultNullInNonNullable,
  #[display(
    fmt = "Data type exceeded: the default value exceeds the maximum value of '{}'",
    raw_type
  )]
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
  #[display(
    fmt = "Mismatched foreign key type: '{}': '{}' (left) and '{}': '{}' (right) are incompatible",
    l_ident,
    l_type,
    r_ident,
    r_type
  )]
  MismatchedForeignKeyType {
    r_ident: String,
    l_ident: String,
    r_type: String,
    l_type: String,
  },
  #[display(fmt = "Invalid foreign key: {}", err)]
  InvalidForeignKey { err: InvalidForeignKeyErr },
  #[display(fmt = "Mismatched composite foreign key")]
  MismatchedCompositeForeignKey,
}

#[derive(Debug, PartialEq, Eq, Clone, Display)]
pub enum InvalidForeignKeyErr {
  #[display(fmt = "the referenced column is neither a primary key or a unique key")]
  NitherUniqueKeyNorPrimaryKey,
  #[display(fmt = "the referenced column is neither a composite primary key or a composite unique key")]
  NitherUniqueKeyNorPrimaryKeyComposite,
  #[display(fmt = "either side of the one-to-one relation must be a primary key or a unique key")]
  One2One,
  #[display(
    fmt = "either side of the composite one-to-one relation must be a composite primary key or a composite unique key"
  )]
  One2OneComposite,
  #[display(fmt = "both sides of the many-to-many relation must be a primary key or a unique key")]
  Many2Many,
  #[display(
    fmt = "both sides of the composite many-to-many relation must be either a composite primary key or a composite unique key"
  )]
  Many2ManyComposite,
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
