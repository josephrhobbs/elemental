//! Main library for the Elemental parser.

mod identifier_parselet;
mod literal_parselet;
mod assignment_parselet;
mod binop_parselet;

use std::collections::HashMap;

use crate::{
    Expression,
    Token,
    TokenClass,
    Tokenizer,
};

use identifier_parselet::IdentifierParselet;
use literal_parselet::LiteralParselet;
use assignment_parselet::AssignmentParselet;
use binop_parselet::BinOpParselet;


/// Abstracts over "prefix parselets".
pub trait PrefixParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression;
}

/// Abstracts over "infix parselets".
pub trait InfixParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token, left: Expression) -> Expression;
}


/// Abstracts over parser behavior.
pub struct Parser {
    prefix_parselets: HashMap<TokenClass, Box<dyn PrefixParselet>>,
    infix_parselets: HashMap<TokenClass, Box<dyn InfixParselet>>,
}

impl Parser {
    /// Constructs a new parser.
    pub fn new() -> Self {
        let mut prefix_parselets: HashMap<TokenClass, Box<dyn PrefixParselet>> = HashMap::new();
        let mut infix_parselets: HashMap<TokenClass, Box<dyn InfixParselet>> = HashMap::new();

        // Declarative grammar begins here.
        prefix_parselets.insert(TokenClass::Identifier, Box::new(IdentifierParselet {}));
        prefix_parselets.insert(TokenClass::Int, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenClass::Float, Box::new(LiteralParselet {}));
        infix_parselets.insert(TokenClass::Assignment, Box::new(AssignmentParselet {}));
        infix_parselets.insert(TokenClass::BinaryOperator, Box::new(BinOpParselet {}));

        Self {
            prefix_parselets,
            infix_parselets,
        }
    }

    /// Parse code into an expression.
    pub fn parse(&self, tokenizer: &mut Tokenizer, _precedence: u8) -> Expression {
        let token = match tokenizer.next() {
            Some(t) => t,
            None => todo!(),
        };

        let parselet: &Box<dyn PrefixParselet> = match self.prefix_parselets.get(&token.get_class()) {
            Some(p) => p,
            None => todo!(),
        };
        let left = parselet.parse(self, tokenizer, token);

        let next = match tokenizer.next() {
            Some(t) => t,
            None => return left,
        };
        let parselet: &Box<dyn InfixParselet> = match self.infix_parselets.get(&next.get_class()) {
            Some(p) => p,
            None => return left,
        };

        parselet.parse(self, tokenizer, next, left)
    }
}