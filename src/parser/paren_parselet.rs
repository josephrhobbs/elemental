//! A parselet for parentheticals.

use crate::parselet_utils::*;

pub struct ParenParselet;

impl PrefixParselet for ParenParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        // Get the expression inside the parenthetical
        parser.parse(tokenizer, token.get_class().into())
    }
}