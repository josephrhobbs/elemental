//! A parselet for numeric literals.

use crate::parselet_utils::*;

pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        // Note: it's ok to use `Result::unwrap()` here because we checked that
        // the token's value parses to valid data during tokenization.
        match token.get_class() {
            TokenClass::Int => Expression::Int (str::parse::<i64>(&token.get_value()).unwrap()),
            TokenClass::Float => Expression::Float (str::parse::<f64>(&token.get_value()).unwrap()),
            _ => unreachable!(),
        }
    }
}