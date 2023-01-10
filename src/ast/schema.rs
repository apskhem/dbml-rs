use std::ops::Range;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct SchemaBlock<'a> {
  pub span_range: Range<usize>,
  /// Input source content.
  pub input: &'a str,
  /// Overall description of the project. This is optional. The file must contain one or zero 'Project' block.
  pub project: Option<project::ProjectBlock>,
  /// Table block.
  pub tables: Vec<table::TableBlock>,
  /// TableGroup block.
  pub table_groups: Vec<table_group::TableGroupBlock>,
  /// Ref block.
  pub refs: Vec<refs::RefBlock>,
  /// Enums block.
  pub enums: Vec<enums::EnumBlock>
}

impl<'a> SchemaBlock<'a> {
  pub fn new(input: &'a str, span_range: Range<usize>) -> Self {
    Self {
      span_range,
      input,
      project: Option::default(),
      tables: Vec::default(),
      table_groups: Vec::default(),
      refs: Vec::default(),
      enums: Vec::default()
    }
  }

  pub fn print(&self) {
    println!("Project:");

    println!("{:?}\n----", self.project);

    println!("Tables:");

    self.tables.iter().for_each(|table| println!("{:?}\n----", table));

    println!("TableGroups:");

    self.table_groups.iter().for_each(|table| println!("{:?}\n----", table));

    println!("Refs:");

    self.refs.iter().for_each(|table| println!("{:?}\n----", table));

    println!("Enums:");

    self.enums.iter().for_each(|table| println!("{:?}\n----", table));
  }
}

