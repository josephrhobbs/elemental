//! Main Elemental processing library.

use std::collections::HashMap;

mod error;
mod tokenizer;
mod parser;
mod standard;
mod expression;

pub use tokenizer::{
    Token,
    TokenClass,
    Tokenizer,
};

pub use parser::Parser;

pub use expression::Expression;


/// Allows parselet files to easily access necessary
/// abstractions without long `use` statements.
/// 
/// For internal use only.
pub mod parselet_utils {
    pub use crate::parser::{
        Parser,
        InfixParselet,
        PrefixParselet,
    };
    pub use crate::tokenizer::{
        Token,
        Tokenizer,
        TokenClass,
    };
    pub use crate::Expression;
}


/// Interprets a `String` of code into an `Expression`.
/// 
/// Also returns a boolean value indicating whether or not the output should be displayed.
pub fn interpret(variables: &mut HashMap<String, Expression>, code: String) -> (Expression, bool) {
    // Create a token stream from the code input.
    let mut tokenizer = Tokenizer::from(code);
    let is_silent = tokenizer.chk_silent();

    // Create a parser and parse from the tokenizer.
    let parser = Parser::new();
    let expression = parser.parse(&mut tokenizer, 0);

    (expression.simplify(variables), is_silent)
}


#[test]
fn interpret_00() {
    let code = "3.1415".to_string();
    println!("{}", interpret(code));
}

#[test]
fn interpret_01() {
    let code = "x = 4".to_string();
    println!("{}", interpret(code));
}