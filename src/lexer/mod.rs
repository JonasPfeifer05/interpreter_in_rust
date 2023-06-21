pub mod token;

use std::collections::HashMap;
use std::fs;
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
    pub fn from_string(program: String) -> Lexer {
        Self {
            program,
            keywords: keyword_map(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Lexer> {
        Ok(Self {
            program: fs::read_to_string(path)?,
            keywords: keyword_map(),
        })
    }

    pub fn lex(&mut self) -> Vec<Token> {
        todo!()
    }
}