#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RefBlock {
  pub rel: Relation,
  pub lhs: Option<RefIdent>,
  pub rhs: RefIdent,
  pub settings: Option<RelationSettings>
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Relation {
  #[default] Undef,
  One2One,
  One2Many,
  Many2One,
  Many2Many
}

impl Relation {
  pub fn match_type(value: &str) -> Result<Self, ()> {
    match value {
      "<" => Ok(Self::One2Many),
      ">" => Ok(Self::Many2One),
      "-" => Ok(Self::One2One),
      "<>" => Ok(Self::Many2Many),
      _ => Err(()),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RefIdent {
  pub schema: Option<String>,
  pub table: String,
  pub compositions: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RelationAction {
  NoAction,
  Cascade,
  Restrict,
  SetNull,
  SetDefault
}

impl RelationAction {
  pub fn match_type(value: &str) -> Self {
    match value {
      "no action" => Self::NoAction,
      "cascade" => Self::Cascade,
      "restrict" => Self::Restrict,
      "set null" => Self::SetNull,
      "set default" => Self::SetDefault,
      _ => unreachable!("'{:?}' type is not supported!", value),
    }
  }
}

impl ToString for RelationAction {
  fn to_string(&self) -> String {
    match self {
      Self::NoAction => format!("no action"),
      Self::Cascade => format!("cascade"),
      Self::Restrict => format!("restrict"),
      Self::SetNull => format!("set null"),
      Self::SetDefault => format!("set default"),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RelationSettings {
  pub on_delete: Option<RelationAction>,
  pub on_update: Option<RelationAction>,
}