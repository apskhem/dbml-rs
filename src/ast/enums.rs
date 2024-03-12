use alloc::string::String;
use alloc::vec::Vec;

use super::*;

#[derive(Debug, Clone, Default)]
pub struct EnumBlock {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub ident: EnumIdent,
  pub values: Vec<EnumValue>,
}

#[derive(Debug, Clone, Default)]
pub struct EnumValue {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub value: Ident,
  pub settings: Option<EnumValueSettings>,
}

#[derive(Debug, Clone, Default)]
pub struct EnumValueSettings {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  /// A vector of key and optional value pairs representing attributes of the enum value.
  pub attributes: Vec<Attribute>,
  /// A note associated with the enum value.
  pub note: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EnumIdent {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub name: Ident,
  pub schema: Option<Ident>,
}
