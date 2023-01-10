#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesBlock {
  pub defs: Vec<IndexesDef>
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesDef {
  pub idents: Vec<IndexesIdent>,
  pub settings: Option<IndexesSettings>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesSettings {
  pub r#type: Option<IndexesType>,
  pub is_unique: bool,
  pub is_pk: bool,
  pub note: Option<String>,
  pub name: Option<String>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IndexesIdent {
  String(String),
  Expr(String)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IndexesType {
  BTree,
  Gin,
  Gist,
  Hash
}

impl IndexesType {
  pub fn match_type(value: &str) -> Self {
    match value {
      "btree" => Self::BTree,
      "gin" => Self::Gin,
      "gist" => Self::Gist,
      "hash" => Self::Hash,
      _ => unreachable!("'{:?}' type is not supported!", value),
    }
  }
}
