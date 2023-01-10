use super::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableGroupBlock {
  pub name: String,
  pub table_idents: Vec<TableGroupIdent>
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableGroupIdent {
  pub schema: Option<String>,
  pub ident_alias: String
}