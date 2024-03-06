use std::ops::Range;

type SpanRange = Range<usize>;

mod enums;
mod indexes;
mod literal;
mod project;
mod refs;
mod schema;
mod table;
mod table_group;

pub use enums::*;
pub use indexes::*;
pub use literal::*;
pub use project::*;
pub use refs::*;
pub use schema::*;
pub use table::*;
pub use table_group::*;
