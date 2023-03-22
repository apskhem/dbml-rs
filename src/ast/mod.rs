use std::ops::Range;

use pest::Span;
use pest::iterators::Pair;
use crate::parser::Rule;

type SpanRange = Range<usize>;

pub mod enums;
pub mod indexes;
pub mod refs;
pub mod project;
pub mod schema;
pub mod table_group;
pub mod table;
