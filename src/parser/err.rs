use super::*;

use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub type ParsingResult<T> = Result<T, Error<Rule>>;

pub fn throw_rules<T>(rules: &[Rule], pair: Pair<Rule>) -> ParsingResult<T> {
  Err(
    Error::new_from_span(
      ErrorVariant::ParsingError { positives: Vec::from(rules), negatives: vec![] },
      pair.as_span()
    )
  )
}

pub fn throw_msg<T>(msg: impl ToString, pair: Pair<Rule>) -> ParsingResult<T> {
  Err(
    Error::new_from_span(
      ErrorVariant::CustomError { message: msg.to_string() },
      pair.as_span()
    )
  )
}