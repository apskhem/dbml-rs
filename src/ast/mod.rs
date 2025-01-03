//! Collections of AST Nodes.

pub(crate) type SpanRange = Range<usize>;

mod enums;
mod indexes;
mod project;
mod refs;
mod schema;
mod table;
mod table_group;
mod unit;

use core::ops::Range;

pub use enums::*;
pub use indexes::*;
pub use project::*;
pub use refs::*;
pub use schema::*;
pub use table::*;
pub use table_group::*;
pub use unit::*;
