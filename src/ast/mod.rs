use std::ops::Range;

use crate::parser::Rule;
use pest::iterators::Pair;
use pest::Span;

type SpanRange = Range<usize>;

pub mod enums;
pub mod indexes;
pub mod project;
pub mod refs;
pub mod schema;
pub mod table;
pub mod table_group;
