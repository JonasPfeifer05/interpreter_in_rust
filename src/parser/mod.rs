use std::iter::Peekable;
use std::vec::IntoIter;
use crate::lexer::token::Token;
use crate::parser::ast::statement::Statement;

pub mod ast;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable()
        }
    }

    pub fn parse_statement(&mut self) -> anyhow::Result<Statement> {
        todo!()
    }
}