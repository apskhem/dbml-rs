#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EnumBlock {
  pub ident: EnumIdent,
  pub values: Vec<EnumValue>
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EnumValue {
  pub value: String,
  pub note: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EnumIdent {
  pub name: String, 
  pub schema: Option<String>
}