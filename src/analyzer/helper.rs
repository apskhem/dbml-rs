use alloc::collections::BTreeSet;

use super::*;

pub(super) fn check_attr_duplicate_keys(attrs: &Vec<Attribute>, input: &str) -> AnalyzerResult<()> {
  let mut acc = BTreeSet::new();
  let dup_keys: Vec<_> = attrs
    .iter()
    .filter(|attr| !acc.insert(&attr.key.to_string))
    .cloned()
    .collect();

  // TODO: handle multiple errs
  if let Some(attr) = dup_keys.first() {
    throw_err(Err::DuplicatedAttributeKey, &attr.span_range, input)?;
  }

  Ok(())
}

pub(super) fn check_prop_duplicate_keys(attrs: &Vec<Property>, input: &str) -> AnalyzerResult<()> {
  let mut acc = BTreeSet::new();
  let dup_keys: Vec<_> = attrs
    .iter()
    .filter(|attr| !acc.insert(&attr.key.to_string))
    .cloned()
    .collect();

  // TODO: handle multiple errs
  if let Some(prop) = dup_keys.first() {
    throw_err(Err::DuplicatedPropertyKey, &prop.span_range, input)?;
  }

  Ok(())
}

pub(super) fn eq_elements<T: Eq + Ord>(lhs: impl Iterator<Item = T>, rhs: impl Iterator<Item = T>) -> bool {
  BTreeSet::from_iter(lhs) == BTreeSet::from_iter(rhs)
}
