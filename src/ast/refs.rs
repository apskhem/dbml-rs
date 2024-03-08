use std::str::FromStr;

use super::SpanRange;

#[derive(Debug, Clone, Default)]
pub struct RefInline {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub rel: Relation,
  pub rhs: RefIdent,
}

#[derive(Debug, Clone, Default)]
pub struct RefBlock {
  /// The range of the span in the source text.
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
  /// Represents '-' one-to-one. E.g: `users.id` - `user_infos.user_id`.
  One2One,
  /// Represents '<' one-to-many. E.g: `users.id` < `posts.user_id`.
  One2Many,
  /// Represents '>' many-to-one. E.g: `posts.user_id` > `users.id`.
  Many2One,
  /// Represents '<>' many-to-many. E.g: `authors.id` <> `books.id`.
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

#[derive(Debug, Clone, Default)]
pub struct RefIdent {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub schema: Option<String>,
  pub table: String,
  pub compositions: Vec<String>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Default)]
pub struct RefSettings {
  /// The range of the span in the source text.
  pub span_range: SpanRange,
  pub on_delete: Option<ReferentialAction>,
  pub on_update: Option<ReferentialAction>,
}
