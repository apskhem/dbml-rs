use std::ops::Range;

use pest::iterators::Pair;
use pest::Span;

use crate::parser::Rule;

type SpanRange = Range<usize>;

pub mod enums;
pub mod indexes;
pub mod project;
pub mod refs;
pub mod schema;
pub mod table;
pub mod table_group;
