//! A parselet for matrix (and vector) input.

use crate::parselet_utils::*;

pub struct MatrixParselet;

impl PrefixParselet for MatrixParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, _token: Token) -> Expression {
        // Create a new vector to hold matrix/vector values
        let mut values: Vec<Expression> = Vec::new();

        // Create a variable to hold column count
        let mut cols: usize = 0;

        let mut current = match tokenizer.peek() {
            Some(t) => t,
            None => todo!(),
        };

        // Count the number of values in the first row (number of columns in the matrix)
        // Note: rows end on a semicolon
        while current.get_class() != TokenClass::Semicolon {
            let expr = parser.parse(tokenizer, 0);
            values.push(expr);
            cols += 1;

            current = match tokenizer.peek() {
                Some(t) => t,
                None => todo!(),
            };
            if current.get_class() == TokenClass::CloseBracket {
                break;
            }
        }

        tokenizer.next();
        current = match tokenizer.peek() {
            Some(t) => t,

            // Stop and return: this is a vector
            None => return Expression::Matrix {
                rows: 1,
                cols,
                values,
            },
        };

        // Fill the remainder of the matrix
        while current.get_class() != TokenClass::CloseBracket {
            let expr = parser.parse(tokenizer, 0);
            values.push(expr);

            current = match tokenizer.peek() {
                Some(t) => t,
                None => break,
            };

            // Discard semicolons
            if current.get_class() == TokenClass::Semicolon {
                tokenizer.next();
            }
        }

        tokenizer.next();

        let rows = values.len()/cols as usize;

        Expression::Matrix {
            rows,
            cols,
            values,
        }
    }
}