//! Main library for the Elemental parser.

use std::collections::HashMap;

use crate::{
    Expression,
    Token,
    TokenClass,
    Tokenizer,
};


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
        let prefix_parselets: HashMap<TokenClass, Box<dyn PrefixParselet>> = HashMap::new();
        let infix_parselets: HashMap<TokenClass, Box<dyn InfixParselet>> = HashMap::new();

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

        parselet.parse(self, tokenizer, token)
    }
}