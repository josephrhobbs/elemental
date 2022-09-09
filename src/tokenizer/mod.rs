//! Provides a tokenizer for the Elemental interpreter.


/// Outlines the types of tokens that Elemental can process.
#[derive(Clone, Copy)]
pub enum TokenClass {

}

/// Holds a token's class and its value.
#[derive(Clone, Copy)]
pub struct Token<'a> {
    class: TokenClass,
    value: &'a str,
}

impl<'a> Token<'a> {
    /// Constructs a new `Token` from a value and a `TokenClass`.
    pub fn new(class: TokenClass, value: &'a str) -> Self {
        Self {
            class,
            value,
        }
    }
}


/// Holds a stream of characters.
pub struct CharStream {
    characters: Vec<char>,
    index: usize,
}

impl CharStream {
    /// Constructs a new character stream from a `String`.
    pub fn from(input: String) -> Self {
        let characters = input.as_str().chars().collect::<Vec<char>>();
        let index = 0;

        Self {
            characters,
            index,
        }
    }

    /// Advances the character stream.
    pub fn next(&mut self) -> Option<char> {
        let character = self.peek();
        if self.index >= self.characters.len() {
            None
        } else {
            self.index += 1;
            character
        }
    }

    /// Peeks at the next character in the stream.
    pub fn peek(&self) -> Option<char> {
        if self.index >= self.characters.len() {
            None
        } else {
            Some (self.characters[self.index])
        }
    }
}


/// Holds a stream of tokens.
pub struct TokenStream<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
}

impl<'a> TokenStream<'a> {
    /// Constructs a new token stream from a `String`.
    pub fn from(input: String) -> Self {
        todo!()
    }

    /// Peeks at the next character in the stream.
    pub fn peek(&self) -> Option<Token> {
        if self.index >= self.tokens.len() {
            None
        } else {
            Some (self.tokens[self.index])
        }
    }

    /// Advances the character stream.
    pub fn next(&mut self) -> Option<Token> {
        self.index += 1;
        self.peek()
    }
}