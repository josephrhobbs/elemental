//! A parselet for parentheticals.

use crate::parselet_utils::*;

pub struct ParenParselet;

impl PrefixParselet for ParenParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        // Get the expression inside the parenthetical
        let expr = parser.parse(tokenizer, token.get_class().into());

        let next = match tokenizer.peek() {
            Some(t) => t,
            None => todo!(),
        };

        if next.get_class() == TokenClass::CloseParen {
            expr
        } else {
            dbg!(&next);
            todo!()
        }
    }
}