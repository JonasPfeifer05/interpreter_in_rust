pub mod token;

use std::collections::HashMap;
use std::path::Path;
use crate::lexer::token::Token;

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
    ])
}

/// Parses a Program into the individual Token
pub struct Lexer {
    program: String,

    // Stores all the keywords
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn from_string() -> Lexer {
        todo!()
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Lexer {
        todo!()
    }

    pub fn lex(&mut self) -> Vec<Token> {
        todo!()
    }
}