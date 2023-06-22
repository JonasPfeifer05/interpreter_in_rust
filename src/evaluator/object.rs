use crate::lexer::token::Token;
use crate::parser::ast::expression::Expression;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i128),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Object>),
    Error(Box<Object>),
    Function {
        parameters: Vec<(String, Token)>,
        typee: Token,
        body: Box<Expression>,
    }
}

impl Object {
    pub fn equal_variant(&self, other: &Object) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}