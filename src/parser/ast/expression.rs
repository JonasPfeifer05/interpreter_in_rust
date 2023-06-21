use crate::lexer::token::Token;

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub enum Expression {
    Identifier {
        name: String,
        value: Box<Expression>
    },
    Integer {
        value: String,
    },
    Float {
        value: String,
    },
    String {
        value: String,
    },
    Boolean {
        value: String,
    },
    Prefix {
        operation: Token,
        value: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operation: Token,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<Expression>,
        alternative: Option<Box<Expression>>,
    },
    While {
        condition: Box<Expression>,
        consequence: Box<Expression>,
    },
    Function {
        name: String,
        parameter: Vec<(String, Token)>,
        body: Box<Expression>,
    },
    Call {
        name: String,
        arguments: Vec<Box<Expression>>,
    },
    Error {
        value: Box<Expression>
    },
    Assign {
        name: String,
        value: Box<Expression>,
    },
    Array {
        values: Vec<Box<Expression>>
    },
    Block {
        expressions: Vec<Box<Expression>>
    },
}