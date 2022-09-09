//! Main Elemental processing library.

mod error;
mod tokenizer;
mod parser;
mod interpreter;
mod standard;


/// Defines the expression types that are available in Elemental.
pub enum Expression {

}


/// Interprets a `String` of code into an `Expression`.
pub fn interpret(code: String) -> Expression {
    todo!()
}