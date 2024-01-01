//! A parselet for assignments.

use crate::parselet_utils::*;
use crate::error::*;

pub struct AssignmentParselet;

impl InfixParselet for AssignmentParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, _token: Token, left: Expression) -> Expression {
        let right = parser.parse(tokenizer, 0);

        let identifier = match left {
            Expression::Identifier (s) => s,
            _ => {
                throw(ExpectedIdentifier);
                return Expression::Nil;
            },
        };

        Expression::Assignment {
            identifier,
            value: Box::new(right),
        }
    }
}