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
  if let Some(dup_key) = dup_keys.first() {
    throw_err(Err::DuplicatedAttributeKey, &dup_key.span_range, input)?;
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