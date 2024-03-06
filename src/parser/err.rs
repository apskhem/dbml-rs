use pest::error::{
  Error,
  ErrorVariant,
};
use pest::iterators::Pair;

use super::*;

pub type ParserResult<T> = Result<T, Error<Rule>>;

pub(super) fn throw_rules<T>(rules: &[Rule], pair: Pair<Rule>) -> ParserResult<T> {
  Err(Error::new_from_span(
    ErrorVariant::ParsingError {
      positives: Vec::from(rules),
      negatives: vec![],
    },
    pair.as_span(),
  ))
}

pub(super) fn throw_msg<T>(msg: impl ToString, pair: Pair<Rule>) -> ParserResult<T> {
  Err(Error::new_from_span(
    ErrorVariant::CustomError {
      message: msg.to_string(),
    },
    pair.as_span(),
  ))
}
