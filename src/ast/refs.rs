use std::str::FromStr;

use super::SpanRange;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RefInline {
  /// The range of the span.
  pub span_range: SpanRange,
  pub rel: Relation,
  pub rhs: RefIdent,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RefBlock {
  /// The range of the span.
  pub span_range: SpanRange,
  pub rel: Relation,
  pub lhs: RefIdent,
  pub rhs: RefIdent,
  pub settings: Option<RefSettings>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Relation {
  #[default]
  Undef,
  One2One,
  One2Many,
  Many2One,
  Many2Many,
}

impl FromStr for Relation {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "<" => Ok(Self::One2Many),
      ">" => Ok(Self::Many2One),
      "-" => Ok(Self::One2One),
      "<>" => Ok(Self::Many2Many),
      _ => Err(format!("incorrect relation symbol '{}'", s)),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RefIdent {
  /// The range of the span.
  pub span_range: SpanRange,
  pub schema: Option<String>,
  pub table: String,
  pub compositions: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReferentialAction {
  NoAction,
  Cascade,
  Restrict,
  SetNull,
  SetDefault,
}

impl FromStr for ReferentialAction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "no action" => Ok(Self::NoAction),
      "cascade" => Ok(Self::Cascade),
      "restrict" => Ok(Self::Restrict),
      "set null" => Ok(Self::SetNull),
      "set default" => Ok(Self::SetDefault),
      _ => Err(()),
    }
  }
}

impl ToString for ReferentialAction {
  fn to_string(&self) -> String {
    match self {
      Self::NoAction => {
        format!("no action")
      }
      Self::Cascade => format!("cascade"),
      Self::Restrict => format!("restrict"),
      Self::SetNull => format!("set null"),
      Self::SetDefault => {
        format!("set default")
      }
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RefSettings {
  /// The range of the span.
  pub span_range: SpanRange,
  pub on_delete: Option<ReferentialAction>,
  pub on_update: Option<ReferentialAction>,
}
