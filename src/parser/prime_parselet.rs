//! A parselet for transpose symbols (`'`).

use crate::parselet_utils::*;

pub struct PrimeParselet;

impl InfixParselet for PrimeParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, _token: Token, left: Expression) -> Expression {
        // Call the transpose function
        Expression::Call {
            name: "t".to_string(),
            args: vec![left],
        }
    }
}