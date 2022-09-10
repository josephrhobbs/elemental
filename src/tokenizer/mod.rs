//! Provides a tokenizer for the Elemental interpreter.


/// Outlines the types of tokens that Elemental can process.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TokenClass {
    Identifier,
    Int,
    Float,
    Assignment,
    Plus,
    Minus,
    Multiply,
    Divide,
    Eq,
    Newline,
    OpenParen,
    CloseParen, 
}

/// Holds a token's class and its value.
#[derive(Clone, Debug)]
pub struct Token {
    class: TokenClass,
    value: String,
}

impl Token {
    /// Constructs a new `Token` from a value and a `TokenClass`.
    pub fn new(class: TokenClass, value: String) -> Self {
        Self {
            class,
            value,
        }
    }

    /// Gets the class of the token.
    pub fn get_class(&self) -> TokenClass {
        self.class
    }

    /// Gets the value of the token.
    pub fn get_value(&self) -> String {
        self.value.to_owned()
    }

    /// Checks if the token is in the given class.
    pub fn check(&self, class: TokenClass) -> bool {
        self.class == class
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

    /// Looks ahead `n` characters.
    /// 
    /// `Self::lookahead(0)` is equivalent to `Self::peek()`.
    pub fn lookahead(&self, n: usize) -> Option<char> {
        if self.index >= self.characters.len() {
            None
        } else {
            Some (self.characters[self.index + n])
        }
    }

    /// Iterates through a stream of characters, pushing characters to a `String`
    /// so long as they are in a given superstring.  Once a character is found that
    /// is not in the given superstring, stops and returns the `String`.
    pub fn get(&mut self, superstring: &str) -> String {
        let mut current = String::new();
        while let Some(c) = self.next() {
            if superstring.contains(c) {
                current.push(c);
            } else {
                break;
            }
        }
        current
    }

    /// Skips comments.
    pub fn skip_comments(&mut self) {
        while self.peek() == Some('/') && self.lookahead(1) == Some('/') {
            while self.peek() != Some('\n') {
                self.next();
            }
            self.next();
        }
    }
}


/// Characters that can compose an identifier.
/// 
/// Please note that, though numbers are included here, identifiers cannot start
/// with a numeric digit (`'0'..='9'`).
const IDENTIFIER: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";


/// Numeric values.  These can compose a numeric literal.
/// 
/// Please note that numeric literals cannot start with `'.'`.
const NUMERIC: &str = "01235456789.";


/// Separators & whitespace.  To be ignored.
const SEPARATORS: &str = " \t";


/// Holds a stream of tokens.
pub struct Tokenizer {
    tokens: Vec<Token>,
    index: usize,
}

impl Tokenizer {
    /// Constructs a new token stream from a `String`.
    pub fn from(input: String) -> Self {
        let index = 0;
        let mut charstream = CharStream::from(input);
        let mut tokens = Vec::new();

        // Skip any comments
        charstream.skip_comments();

        while let Some(c) = charstream.next() {
            if SEPARATORS.contains(c) {
                continue;
            }

            let token = match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let name = format!(
                        "{}{}",
                        c,
                        charstream.get(IDENTIFIER),
                    );
                    Token::new(TokenClass::Identifier, name)
                },
                '0'..='9' => {
                    let raw = format!(
                        "{}{}",
                        c,
                        charstream.get(NUMERIC),
                    );
                    
                    let token = match str::parse::<i64>(&raw) {
                        Ok(_) => Token::new(TokenClass::Int, raw),
                        Err(_) => match str::parse::<f64>(&raw) {
                            Ok(_) => Token::new(TokenClass::Float, raw),
                            Err(_) => todo!(),
                        },
                    };
                    token
                },
                '=' => if charstream.peek() == Some('=') {
                    Token::new(TokenClass::Eq, "==".to_string())
                } else if let Some(_) = charstream.peek() {
                    Token::new(TokenClass::Assignment, "=".to_string())
                } else {
                    todo!()
                },
                '\n' => Token::new(TokenClass::Newline, '\n'.to_string()),
                '+' => Token::new(TokenClass::Plus, '+'.to_string()),
                '-' => Token::new(TokenClass::Minus, '-'.to_string()),
                '*' => Token::new(TokenClass::Multiply, '*'.to_string()),
                '/' => Token::new(TokenClass::Divide, '/'.to_string()),
                '(' => Token::new(TokenClass::OpenParen, '('.to_string()),
                ')' => Token::new(TokenClass::CloseParen, ')'.to_string()),
                _ => todo!(),
            };
            tokens.push(token);

            // Skip comments
            charstream.skip_comments();
        }

        Self {
            tokens,
            index,
        }
    }

    /// Peeks at the next character in the stream.
    pub fn peek(&self) -> Option<Token> {
        if self.index >= self.tokens.len() {
            None
        } else {
            Some (self.tokens[self.index].to_owned())
        }
    }

    /// Advances the character stream.
    pub fn next(&mut self) -> Option<Token> {
        let token = self.peek();
        self.index += 1;
        token
    }

    /// Returns all tokens without consuming the tokenizer.
    pub fn get_tokens(&mut self) -> Vec<Token> {
        self.tokens.to_owned()
    }

    /// Get the precedence of the next token.
    pub fn get_next_precedence(&mut self) -> u8 {
        if let Some(t) = self.peek() {
            t.class.into()
        } else {
            0
        }
    }
}

#[test]
fn tokenize_00() {
    let input: String = "x = 1.3\ny = 2.6".to_string();
    let mut tokenizer = Tokenizer::from(input);
    println!("Tokens: {:#?}", tokenizer.get_tokens());
}