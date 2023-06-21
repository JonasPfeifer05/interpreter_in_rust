pub mod token;

use std::collections::HashMap;
use std::fs;
use std::iter::Peekable;
use std::path::Path;
use std::vec::IntoIter;
use crate::lexer::token::Token;

const SKIPPABLES: [char; 4] = ['\n', '\r', '\t', ' '];

pub fn keyword_map() -> HashMap<String, Token> {
    HashMap::from([
        ("true".to_string(), Token::Boolean("true".to_string())),
        ("false".to_string(), Token::Boolean("false".to_string())),
        ("function".to_string(), Token::Function),
        ("let".to_string(), Token::Let),
        ("int".to_string(), Token::IntegerType),
        ("float".to_string(), Token::FloatType),
        ("string".to_string(), Token::StringType),
        ("bool".to_string(), Token::BooleanType),
        ("if".to_string(), Token::If),
        ("while".to_string(), Token::While),
        ("ret".to_string(), Token::Return),
        ("err".to_string(), Token::Error),
    ])
}

/// Parses a Program into the individual Token
pub struct Lexer {
    // String representation of the program
    pub program: Peekable<IntoIter<char>>,

    // Stores all the keywords
    pub keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn from_string(program: String) -> Lexer {
        Self {
            program: program.chars().collect::<Vec<char>>().into_iter().peekable(),
            keywords: keyword_map(),
        }
    }

    #[allow(unused)]
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Lexer> {
        Ok(Self {
            program: fs::read_to_string(path)?.chars().collect::<Vec<char>>().into_iter().peekable(),
            keywords: keyword_map(),
        })
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(char) = self.program.next() {
            if SKIPPABLES.contains(&char) { continue }
            let token = match char {
                // Types
                c if c.is_ascii_alphabetic() => self.parse_identifier(c),
                c if c.is_digit(10) => self.parse_number(c),
                c if c == '"' => self.parse_string(),

                // Special characters
                ';' => Token::Semicolon,
                ':' => Token::Colon,
                ',' => Token::Comma,
                '.' => Token::Dot,

                // Brackets
                '(' => Token::LParent,
                ')' => Token::RParent,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '[' => Token::LBracket,
                ']' => Token::RBracket,

                // Operators
                '=' => if let Some(&'=') = self.program.peek() {
                    self.program.next();
                    Token::Equal
                } else { Token::Assign },
                '+' => Token::Add,
                '-' => Token::Subtract,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '%' => Token::Modular,

                '!' => if let Some(&'=') = self.program.peek() {
                    self.program.next();
                    Token::NotEqual
                } else { Token::Invert },
                '&' if matches!(self.program.peek(), Some(&'&')) => {
                    self.program.next();
                    Token::And
                }
                '|' if matches!(self.program.peek(), Some(&'|')) => {
                    self.program.next();
                    Token::Or
                }
                '<' => if let Some(&'=') = self.program.peek() {
                    self.program.next();
                    Token::LessThanEqual
                } else { Token::LessThan },
                '>' => if let Some(&'=') = self.program.peek() {
                    self.program.next();
                    Token::GreaterThanEqual
                } else { Token::GreaterThan },

                _ => Token::Illegal,
            };
            tokens.push(token);
        }

        tokens
    }

    pub fn parse_identifier(&mut self, c: char) -> Token {
        let mut identifier = String::from(c);

        while let Some(c) = self.program.peek() {
            if !c.is_ascii_alphabetic() { break }
            identifier.push(*c);
            self.program.next();
        }

        if let Some(keyword) = self.keywords.get(&identifier) {
            return keyword.clone();
        }

        Token::Identifier(identifier)
    }

    pub fn parse_number(&mut self, c: char) -> Token {
        let mut number = String::from(c);

        while let Some(c) = self.program.peek() {
            if !(c.is_digit(10) || c == &'.') { break }
            number.push(*c);
            self.program.next();
        }

        if number.contains(".") { return Token::Float(number) }

        Token::Integer(number)
    }

    pub fn parse_string(&mut self) -> Token {
        let mut string = String::new();

        while let Some(c) = self.program.next() {
            if c == '"' { break }
            string.push(c);
        }

        Token::String(string)
    }
}