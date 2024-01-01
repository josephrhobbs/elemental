//! A parselet for identifiers.

use crate::parselet_utils::*;

pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        Expression::Identifier (token.get_value())
    }
}