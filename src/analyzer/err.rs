use alloc::string::{
  String,
  ToString,
};
use core::ops::Range;

use derive_more::Display;
use pest::Span;
use pest::error::{
  Error,
  ErrorVariant,
};

use crate::parser::Rule;

pub type AnalyzerResult<T> = Result<T, Error<Rule>>;

#[derive(Debug, PartialEq, Eq, Clone, Display)]
pub enum Err {
  #[display("Nullable primary key: a nullable primary is not allowed")]
  NullablePrimaryKey,
  #[display("Array primary key: an array with primary is not allowed")]
  ArrayPrimaryKey,
  #[display("Invalid indexes setting: can specify either 'pk', 'unique', or 'type' within a setting")]
  InvalidIndexesSetting,
  #[display("Duplicate attribute key")]
  DuplicateAttributeKey,
  #[display("Duplicate property key")]
  DuplicatePropertyKey,
  #[display("Duplicate project setting")]
  DuplicateProjectSetting,
  #[display("Duplicate primary key")]
  DuplicatePrimaryKey,
  #[display("Duplicate unique key")]
  DuplicateUniqueKey,
  #[display("Duplicate index key")]
  DuplicateIndexKey,
  #[display("Duplicate table name")]
  DuplicateTableName,
  #[display("Duplicate column name")]
  DuplicateColumnName,
  #[display("Conflict relation")]
  ConflictRelation,
  #[display("Duplicate enum name")]
  DuplicateEnumName,
  #[display("Duplicate enum value")]
  DuplicateEnumValue,
  #[display("Duplicate table group name")]
  DuplicateTableGroupName,
  #[display("Duplicate table group item")]
  DuplicateTableGroupItem,
  #[display("Duplicate alias")]
  DuplicateAlias,
  #[display("Conflict nullable setting: 'null' and 'not null' must not appear within the same setting")]
  ConflictNullableSetting,
  #[display("Empty indexes block")]
  EmptyIndexesBlock,
  #[display("Invalid enum")]
  InvalidEnum,
  #[display("Invalid data type")]
  InvalidDataType,
  #[display("Invalid data type arguments: '{}' requires only {} argument(s)", raw_type, n_arg)]
  InvalidDataTypeArguments { raw_type: String, n_arg: u32 },
  #[display("Invalid argument value: the value for the argument is not integer")]
  InvalidArgumentValue,
  #[display(
    "Invalid default value: the defualt value '{}' is not associated with '{}'",
    raw_value,
    raw_type
  )]
  InvalidDefaultValue { raw_value: String, raw_type: String },
  #[display("Default null in non-nullable: the default value cannot be null in non-nullable field")]
  DefaultNullInNonNullable,
  #[display("Data type exceeded: the default value exceeds the maximum value of '{}'", raw_type)]
  DataTypeExceeded { raw_type: String },
  #[display("Table group not found")]
  TableGroupNotFound,
  #[display("Schema not found")]
  SchemaNotFound,
  #[display("Table not found")]
  TableNotFound,
  #[display("Column not found")]
  ColumnNotFound,
  #[display("Enum not found")]
  EnumNotFound,
  #[display("Enum value not found")]
  EnumValueNotFound,
  #[display(
    "Mismatched foreign key type: '{}': '{}' (left) and '{}': '{}' (right) are incompatible",
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
  #[display("Invalid foreign key: {}", err)]
  InvalidForeignKey { err: InvalidForeignKeyErr },
  #[display("Mismatched composite foreign key")]
  MismatchedCompositeForeignKey,
}

#[derive(Debug, PartialEq, Eq, Clone, Display)]
pub enum InvalidForeignKeyErr {
  #[display("the referenced column is neither a primary key or a unique key")]
  NitherUniqueKeyNorPrimaryKey,
  #[display("the referenced column is neither a composite primary key or a composite unique key")]
  NitherUniqueKeyNorPrimaryKeyComposite,
  #[display("either side of the one-to-one relation must be a primary key or a unique key")]
  One2One,
  #[display(
    "either side of the composite one-to-one relation must be a composite primary key or a composite unique key"
  )]
  One2OneComposite,
  #[display("both sides of the many-to-many relation must be a primary key or a unique key")]
  Many2Many,
  #[display(
    "both sides of the composite many-to-many relation must be either a composite primary key or a composite unique key"
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
