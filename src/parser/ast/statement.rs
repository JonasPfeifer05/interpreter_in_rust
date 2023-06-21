use crate::parser::ast::expression::Expression;

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        value: Box<Expression>,
    },
    Return {
        value: Box<Expression>,
    },
    Expression {
        value: Box<Expression>,
    },
}