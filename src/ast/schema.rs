use alloc::vec::Vec;

use super::*;

/// Represents the entire structure of a parsed DBML file.
#[derive(Debug, Clone, Default)]
pub struct SchemaBlock<'a> {
  /// The span range of the entire parsed DBML structure in the source text.
  pub span_range: SpanRange,
  /// The input source content.
  pub input: &'a str,
  /// A vector of top-level blocks comprising the parsed DBML structure.
  pub blocks: Vec<TopLevelBlock>,
}

/// An enum representing various top-level blocks in a parsed DBML file.
#[derive(Debug, Clone)]
pub enum TopLevelBlock {
  /// Represents a project block in the DBML file.
  Project(ProjectBlock),
  /// Represents a table block in the DBML file.
  Table(TableBlock),
  /// Represents a table group block in the DBML file.
  TableGroup(TableGroupBlock),
  /// Represents a note block in the DBML file.
  Note(NoteBlock),
  /// Represents a reference block in the DBML file.
  Ref(RefBlock),
  /// Represents an enum block in the DBML file.
  Enum(EnumBlock),
}

impl<'a> SchemaBlock<'a> {
  pub fn project(&self) -> Vec<&ProjectBlock> {
    self
      .blocks
      .iter()
      .filter_map(|block| {
        if let TopLevelBlock::Project(project) = block {
          Some(project)
        } else {
          None
        }
      })
      .collect()
  }

  pub fn tables(&self) -> Vec<&TableBlock> {
    self
      .blocks
      .iter()
      .filter_map(|block| {
        if let TopLevelBlock::Table(table) = block {
          Some(table)
        } else {
          None
        }
      })
      .collect()
  }

  pub fn table_groups(&self) -> Vec<&TableGroupBlock> {
    self
      .blocks
      .iter()
      .filter_map(|block| {
        if let TopLevelBlock::TableGroup(table_group) = block {
          Some(table_group)
        } else {
          None
        }
      })
      .collect()
  }

  pub fn refs(&self) -> Vec<&RefBlock> {
    self
      .blocks
      .iter()
      .filter_map(|block| {
        if let TopLevelBlock::Ref(reference) = block {
          Some(reference)
        } else {
          None
        }
      })
      .collect()
  }

  pub fn enums(&self) -> Vec<&EnumBlock> {
    self
      .blocks
      .iter()
      .filter_map(|block| {
        if let TopLevelBlock::Enum(enum_block) = block {
          Some(enum_block)
        } else {
          None
        }
      })
      .collect()
  }
}
