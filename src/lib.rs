//! Main Elemental processing library.

use std::fmt::{
    Display,
    Result,
    Formatter,
};

mod error;
mod tokenizer;
mod parser;
mod interpreter;
mod standard;

pub use tokenizer::{
    Token,
    TokenClass,
    Tokenizer,
};

pub use parser::Parser;


/// Defines the expression types that are available in Elemental.
pub enum Expression {
    Assignment {
        identifier: String,
        value: Box<Expression>,
    },
    Identifier (String),
    Int (i64),
    Float (f64),
    BinOp {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
}

/// Implementing `std::fmt::Display` allows us to print expressions
/// using the default formatter.
impl Display for Expression {
    /// Display each `Expression`.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "hello there")
    }
}


/// Interprets a `String` of code into an `Expression`.
pub fn interpret(code: String) -> Expression {
    // Create a token stream from the code input.
    let mut tokenizer = Tokenizer::from(code);

    // Create a parser and parse from the tokenizer.
    let parser = Parser::new();
    let expression = parser.parse(&mut tokenizer, 0);

    expression
}