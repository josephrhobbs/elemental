//! A parselet for binary operations.

use crate::parselet_utils::*;

pub struct BinOpParselet;

impl InfixParselet for BinOpParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token, left: Expression) -> Expression {
        let right = parser.parse(tokenizer, 0);

        Expression::BinOp {
            left: Box::new(left),
            op: token.get_value(),
            right: Box::new(right),
        }
    }
}