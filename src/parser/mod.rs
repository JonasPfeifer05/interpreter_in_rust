use std::iter::Peekable;
use std::vec::IntoIter;
use anyhow::bail;
use crate::error::ParseError::ExpectedButFound;
use crate::lexer::token::Token;
use crate::parser::ast::expression::Expression;
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

    pub fn parse(&mut self) -> anyhow::Result<Vec<Statement>> {
        let mut statements = vec![];

        while self.tokens.peek().is_some() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    pub fn parse_statement(&mut self) -> anyhow::Result<Statement> {
        match self.tokens.peek().unwrap() {
            Token::Let => self.parse_let_statement(),
            Token::Function => self.parse_function_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_let_statement(&mut self) -> anyhow::Result<Statement> {
        self.tokens.next();

        let name = match self.tokens.next().ok_or(ExpectedButFound("Identifier".to_string(), Token::EOF))? {
            Token::Identifier(val) => val,
            token => bail!(ExpectedButFound("Identifier".to_string(), token)),
        };

        let value = Box::new(self.parse_expression()?);

        Ok(Statement::Let {
            name,
            value
        })
    }

    pub fn parse_return_statement(&mut self) -> anyhow::Result<Statement> {
        self.tokens.next();

        let value = Box::new(self.parse_expression()?);

        Ok(Statement::Return {
            value
        })
    }

    pub fn parse_function_statement(&mut self) -> anyhow::Result<Statement> {
        self.tokens.next();
        todo!()
    }

    pub fn parse_expression_statement(&mut self) -> anyhow::Result<Statement> {
        todo!()
    }

    pub fn parse_expression(&mut self) -> anyhow::Result<Expression> {
        todo!()
    }
}