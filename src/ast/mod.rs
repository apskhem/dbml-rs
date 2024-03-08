use std::ops::Range;

pub(crate) type SpanRange = Range<usize>;

mod enums;
mod indexes;
mod meta;
mod unit;
mod project;
mod refs;
mod schema;
mod table;
mod table_group;

pub use enums::*;
pub use indexes::*;
pub use meta::*;
pub use unit::*;
pub use project::*;
pub use refs::*;
pub use schema::*;
pub use table::*;
pub use table_group::*;
