use alloc::string::String;
use alloc::vec::Vec;

use super::*;

/// Represents a top-level block of enum.
#[derive(Debug, Clone, Default)]
pub struct EnumBlock {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The identifier of the enum block including schema (optional) and name.
  pub ident: EnumIdent,
  /// The list of variants of the enums.
  pub values: Vec<EnumValue>,
}

/// Represents an enum value or variant.
#[derive(Debug, Clone, Default)]
pub struct EnumValue {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub value: Ident,
  pub settings: Option<EnumValueSettings>,
}

/// Represents settings of an enum value or variant.
#[derive(Debug, Clone, Default)]
pub struct EnumValueSettings {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// A vector of key and optional value pairs representing attributes of the enum value.
  pub attributes: Vec<Attribute>,
  /// A note associated with the enum value.
  pub note: Option<String>,
}

/// Represents an enum identifier including schema (optional) and name.
#[derive(Debug, Clone, Default)]
pub struct EnumIdent {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// The schema of the enum.
  pub schema: Option<Ident>,
  /// The name of the enum.
  pub name: Ident,
}
