use crate::lexer::token::Token;
use crate::parser::ast::expression::Expression;

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        typee: Token,
        value: Box<Expression>,
    },
    Return {
        value: Box<Expression>,
    },
    Expression {
        value: Box<Expression>,
    },
    Function {
        name: String,
        parameter: Vec<(String, Token)>,
        body: Box<Expression>,
    },
}