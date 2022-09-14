//! Main library for the Elemental parser.

mod identifier_parselet;
mod literal_parselet;
mod assignment_parselet;
mod binop_parselet;
mod paren_parselet;
mod matrix_parselet;
mod func_parselet;
mod prime_parselet;

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
use paren_parselet::ParenParselet;
use matrix_parselet::MatrixParselet;
use func_parselet::FuncParselet;
use prime_parselet::PrimeParselet;


/// Converts a token class into a precedence value.
impl From<TokenClass> for u8 {
    fn from(t: TokenClass) -> u8 {
        match t {
            TokenClass::Assignment => 1,
            TokenClass::Plus => 3,
            TokenClass::Minus => 3,
            TokenClass::Multiply => 4,
            TokenClass::Divide => 4,
            TokenClass::Prime => 5,
            TokenClass::OpenParen => 6,
            TokenClass::OpenBracket => 7,
            _ => 0,
        }
    }
}


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
        prefix_parselets.insert(TokenClass::OpenParen, Box::new(ParenParselet {}));
        prefix_parselets.insert(TokenClass::OpenBracket, Box::new(MatrixParselet {}));
        infix_parselets.insert(TokenClass::Assignment, Box::new(AssignmentParselet {}));
        infix_parselets.insert(TokenClass::Plus, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenClass::Minus, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenClass::Multiply, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenClass::Divide, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenClass::OpenParen, Box::new(FuncParselet {}));
        infix_parselets.insert(TokenClass::Prime, Box::new(PrimeParselet {}));

        Self {
            prefix_parselets,
            infix_parselets,
        }
    }

    /// Parse code into an expression.
    pub fn parse(&self, tokenizer: &mut Tokenizer, precedence: u8) -> Expression {
        let token = match tokenizer.next() {
            Some(t) => t,
            None => return Expression::Nil,
        };

        let parselet: &Box<dyn PrefixParselet> = match self.prefix_parselets.get(&token.get_class()) {
            Some(p) => p,
            None => return Expression::Nil,
        };
        let mut left = parselet.parse(self, tokenizer, token);

        while precedence < tokenizer.get_next_precedence() {
            let next = match tokenizer.peek() {
                Some(t) => t,
                None => break,
            };
            
            let parselet: &Box<dyn InfixParselet> = match self.infix_parselets.get(&next.get_class()) {
                Some(p) => p,
                None => break,
            };
            tokenizer.next();

            left = parselet.parse(self, tokenizer, next, left);
        }

        left
    }
}