//! A parselet for function calls.

use crate::parselet_utils::*;
use crate::error::*;

pub struct FuncParselet;

impl InfixParselet for FuncParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, _token: Token, left: Expression) -> Expression {
        if let Expression::Identifier (s) = left {
            let mut args = Vec::new();

            // Construct a list of function arguments

            let mut current = match tokenizer.peek() {
                Some(t) => t,
                None => {
                    throw(UnexpectedEof);
                    return Expression::Nil;
                },
            };

            while current.get_class() != TokenClass::CloseParen {
                let arg = parser.parse(tokenizer, current.get_class().into());
                args.push(arg);

                current = match tokenizer.peek() {
                    Some(t) => t,
                    None => break,
                };

                if current.get_class() == TokenClass::Comma {
                    tokenizer.next();
                }
            }

            // Consume the closing parenthesis
            tokenizer.next();

            Expression::Call {
                name: s,
                args,
            }
        } else {
            throw(ExpectedIdentifier);
            return Expression::Nil;
        }
    }
}