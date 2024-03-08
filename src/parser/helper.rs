use std::ops::Range;

use pest::Span;

/// Coverts span into range (usize).
pub(super) fn s2r(span: Span) -> Range<usize> {
  span.start()..span.end()
}
