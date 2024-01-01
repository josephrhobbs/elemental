//! A parselet for binary operations.

use crate::parselet_utils::*;
use crate::error::*;

pub struct BinOpParselet;

impl InfixParselet for BinOpParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token, left: Expression) -> Expression {
        let _peek = match tokenizer.peek() {
            Some(t) => t,
            None => {
                throw(UnexpectedEof);
                return Expression::Nil;
            },
        };

        let right = parser.parse(tokenizer, token.get_class().into());

        Expression::BinOp {
            left: Box::new(left),
            op: token.get_value(),
            right: Box::new(right),
        }
    }
}