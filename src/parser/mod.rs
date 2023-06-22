use std::iter::Peekable;
use std::vec::IntoIter;
use anyhow::bail;
use crate::error::ParseError::{ExpectedButFound, ExpectedButFoundExpression, ExpectedTokenButFound, RanOutOfTokens, UnexpectedTokenFound};
use crate::lexer::token::Token;
use crate::parser::ast::expression::Expression;
use crate::parser::ast::statement::Statement;
use crate::parser::precedences::Precedences;

pub mod ast;
pub mod precedences;

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

        let name = match self.tokens.next().ok_or(RanOutOfTokens)? {
            Token::Identifier(val) => val,
            token => bail!(ExpectedButFound("Identifier".to_string(), token)),
        };

        self.assert_next_token(Token::Colon)?;

        let typee = self.tokens.next().ok_or(RanOutOfTokens).unwrap();

        match typee {
            Token::IntegerType => {}
            Token::FloatType => {}
            Token::StringType => {}
            Token::BooleanType => {}
            token => bail!(ExpectedButFound("Type".to_string(), token))
        };

        self.assert_next_token(Token::Assign)?;

        let value = Box::new(self.parse_expression(Precedences::Lowest)?);

        self.assert_next_token(Token::Semicolon)?;

        Ok(Statement::Let {
            name,
            typee,
            value,
        })
    }

    pub fn parse_return_statement(&mut self) -> anyhow::Result<Statement> {
        self.tokens.next();

        let value = Box::new(self.parse_expression(Precedences::Lowest)?);

        self.assert_next_token(Token::Semicolon)?;

        Ok(Statement::Return {
            value
        })
    }

    pub fn parse_function_statement(&mut self) -> anyhow::Result<Statement> {
        self.tokens.next();

        let name = match self.tokens.next().ok_or(RanOutOfTokens)? {
            Token::Identifier(val) => val,
            token => bail!(ExpectedButFound("Identifier".to_string(), token))
        };

        self.assert_next_token(Token::LParent)?;

        let mut parameter = vec![];

        if let Some(&Token::RParent) = self.tokens.peek() {}
        else {
            let name = match self.tokens.next().ok_or(RanOutOfTokens)? {
                Token::Identifier(val) => val,
                token => bail!(ExpectedButFound("Identifier".to_string(), token))
            };
            self.assert_next_token(Token::Colon)?;
            let typee = self.tokens.next().ok_or(RanOutOfTokens)?;
            match typee {
                Token::IntegerType |
                Token::FloatType |
                Token::StringType |
                Token::BooleanType => {}
                token => bail!(ExpectedButFound("Type".to_string(), token))
            }
            parameter.push((name, typee));

            while let Some(token) = self.tokens.peek() {
                println!("{:?}", token);
                if token.equal_variant(&Token::RParent) { break }

                self.assert_next_token(Token::Comma)?;

                let name = match self.tokens.next().ok_or(RanOutOfTokens)? {
                    Token::Identifier(val) => val,
                    token => bail!(ExpectedButFound("Identifier".to_string(), token))
                };

                self.assert_next_token(Token::Colon)?;

                let typee = self.tokens.next().ok_or(RanOutOfTokens)?;

                match typee {
                    Token::IntegerType |
                    Token::FloatType |
                    Token::StringType |
                    Token::BooleanType => {}
                    token => bail!(ExpectedButFound("Type".to_string(), token))
                }

                parameter.push((name, typee));
            }
        }

        self.assert_next_token(Token::RParent)?;

        let body = Box::new(self.parse_block_expression()?);

        Ok(Statement::Function {
            name,
            parameter,
            body,
        })
    }

    pub fn parse_expression_statement(&mut self) -> anyhow::Result<Statement> {
        let value = Box::new(self.parse_expression(Precedences::Lowest)?);

        if let Some(&Token::Semicolon) = self.tokens.peek() {
            self.assert_next_token(Token::Semicolon)?;
        }

        Ok(Statement::Expression {
            value
        })
    }

    pub fn parse_expression(&mut self, precedences: Precedences) -> anyhow::Result<Expression> {
        let mut left_expr = match self.tokens.next().ok_or(RanOutOfTokens)? {
            Token::Identifier(name) => Expression::Identifier { name },
            Token::Integer(value) => Expression::Integer { value },
            Token::Float(value) => Expression::Float { value },
            Token::String(value) => Expression::String { value },
            Token::Boolean(value) => Expression::Boolean { value },
            Token::Subtract => self.parse_prefix_expression(Token::Subtract)?,
            Token::Invert => self.parse_prefix_expression(Token::Invert)?,
            Token::LParent => self.parse_grouped_expression()?,
            Token::If => self.parse_if_expression()?,
            Token::While => self.parse_while_expression()?,
            Token::LBracket => self.parse_array_expression()?,
            Token::Error => self.parse_error_expression()?,
            token => bail!(UnexpectedTokenFound(token))
        };

        while let Some(token) = self.tokens.peek() {
            if token.equal_variant(&Token::Semicolon) || precedences >= token.precedence() { break; }
            let token = self.tokens.next().unwrap();
            let infix = match token {
                Token::Add => self.parse_infix_expression(left_expr, Token::Add),
                Token::Subtract => self.parse_infix_expression(left_expr, Token::Subtract),
                Token::Multiply => self.parse_infix_expression(left_expr, Token::Multiply),
                Token::Divide => self.parse_infix_expression(left_expr, Token::Divide),
                Token::Modular => self.parse_infix_expression(left_expr, Token::Modular),
                Token::Equal => self.parse_infix_expression(left_expr, Token::Equal),
                Token::NotEqual => self.parse_infix_expression(left_expr, Token::NotEqual),
                Token::LessThan => self.parse_infix_expression(left_expr, Token::LessThan),
                Token::GreaterThan => self.parse_infix_expression(left_expr, Token::GreaterThan),
                Token::LessThanEqual => self.parse_infix_expression(left_expr, Token::LessThanEqual),
                Token::GreaterThanEqual => self.parse_infix_expression(left_expr, Token::GreaterThanEqual),
                Token::Or => self.parse_infix_expression(left_expr, Token::Or),
                Token::And => self.parse_infix_expression(left_expr, Token::And),
                Token::LParent => self.parse_call_expression(left_expr),
                Token::Assign => self.parse_assign_expression(left_expr),
                _ => { return Ok(left_expr); }
            }?;

            left_expr = infix;
        }

        Ok(left_expr)
    }

    pub fn parse_grouped_expression(&mut self) -> anyhow::Result<Expression> {
        let expr = self.parse_expression(Precedences::Lowest);

        self.assert_next_token(Token::RParent)?;

        return expr;
    }

    pub fn parse_array_expression(&mut self) -> anyhow::Result<Expression> {
        if let Some(&Token::RBracket) = self.tokens.peek() {
            return Ok(Expression::Array {
                values: vec![],
            });
        }

        let mut values = vec![Box::new(self.parse_expression(Precedences::Lowest)?)];

        while let Some(token) = self.tokens.peek() {
            if token.equal_variant(&Token::RBracket) {
                self.tokens.next();
                break;
            }
            self.assert_next_token(Token::Comma)?;
            values.push(Box::new(self.parse_expression(Precedences::Lowest)?));
        }

        Ok(Expression::Array {
            values
        })
    }

    pub fn parse_prefix_expression(&mut self, prefix: Token) -> anyhow::Result<Expression> {
        let value = Box::new(self.parse_expression(Precedences::Prefix)?);
        Ok(Expression::Prefix {
            prefix,
            value,
        })
    }

    pub fn parse_infix_expression(&mut self, left: Expression, infix: Token) -> anyhow::Result<Expression> {
        let precedence = infix.precedence();
        let right = self.parse_expression(precedence)?;
        Ok(Expression::Infix {
            left: Box::new(left),
            operation: infix,
            right: Box::new(right),
        })
    }

    pub fn parse_assign_expression(&mut self, left: Expression) -> anyhow::Result<Expression> {
        let name = match left {
            Expression::Identifier { name } => name,
            expr => bail!(ExpectedButFoundExpression("Identifier".to_string(), expr))
        };

        let value = Box::new(self.parse_expression(Precedences::Lowest)?);

        Ok(Expression::Assign {
            name,
            value,
        })
    }

    pub fn parse_call_expression(&mut self, left: Expression) -> anyhow::Result<Expression> {
        let name = match left {
            Expression::Identifier { name } => name,
            expr => bail!(ExpectedButFoundExpression("Identifier".to_string(), expr))
        };

        let mut arguments = vec![];
        if let Some(&Token::RParent) = self.tokens.peek() {}
        else {
            arguments.push(Box::new(self.parse_expression(Precedences::Lowest)?));
            while let Some(token) = self.tokens.peek() {
                if token.equal_variant(&Token::RParent) { break; }
                self.assert_next_token(Token::Comma)?;
                arguments.push(Box::new(self.parse_expression(Precedences::Lowest)?));
            }
        }

        self.assert_next_token(Token::RParent)?;

        Ok(Expression::Call {
            name,
            arguments,
        })
    }

    pub fn parse_if_expression(&mut self) -> anyhow::Result<Expression> {
        self.assert_next_token(Token::LParent)?;

        let condition = Box::new(self.parse_expression(Precedences::Lowest)?);

        self.assert_next_token(Token::RParent)?;

        let consequence = Box::new(self.parse_block_expression()?);

        if let Some(&Token::Else) = self.tokens.peek() {
            self.assert_next_token(Token::Else)?;
            let alternative = Box::new(self.parse_block_expression()?);
            Ok(Expression::If {
                condition,
                consequence,
                alternative: Some(alternative),
            })
        } else {
            Ok(Expression::If {
                condition,
                consequence,
                alternative: None,
            })
        }
    }

    pub fn parse_while_expression(&mut self) -> anyhow::Result<Expression> {
        self.assert_next_token(Token::LParent)?;

        let condition = Box::new(self.parse_expression(Precedences::Lowest)?);

        self.assert_next_token(Token::RParent)?;

        let consequence = Box::new(self.parse_block_expression()?);

        Ok(Expression::While {
            condition,
            consequence,
        })
    }

    pub fn parse_block_expression(&mut self) -> anyhow::Result<Expression> {
        self.assert_next_token(Token::LBrace)?;

        let mut statements = vec![];

        while let Some(token) = self.tokens.peek() {
            if token.equal_variant(&Token::RBrace) {
                self.tokens.next();
                break;
            }
            statements.push(Box::new(self.parse_statement()?));
        }

        Ok(Expression::Block {
            statements
        })
    }

    pub fn parse_error_expression(&mut self) -> anyhow::Result<Expression> {
        self.assert_next_token(Token::LParent)?;

        let value = Box::new(self.parse_expression(Precedences::Lowest)?);

        self.assert_next_token(Token::RParent)?;

        Ok(Expression::Error {
            value
        })
    }

    pub fn assert_next_token(&mut self, token: Token) -> anyhow::Result<()> {
        if let Some(cur) = self.tokens.next() {
            if cur != token { bail!(ExpectedTokenButFound(token, cur)) }
        } else { bail!(RanOutOfTokens) }
        Ok(())
    }
}