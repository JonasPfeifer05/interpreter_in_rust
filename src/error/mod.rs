use thiserror::Error;
use crate::evaluator::object::Object;
use crate::lexer::token::Token;
use crate::parser::ast::expression::Expression;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parser expected token but ran out of tokens!")]
    RanOutOfTokens,
    #[error("Parser expected {0} but found {1:?}!")]
    ExpectedButFound(String, Token),
    #[error("Parser expected {0} but found {1:?}!")]
    ExpectedButFoundExpression(String, Expression),
    #[error("Parser expected {0:?} but found {1:?}!")]
    ExpectedTokenButFound(Token, Token),
    #[error("Found unexpected token: {0:?}!")]
    UnexpectedTokenFound(Token),
}

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Expected {0:?} but found {1:?}!")]
    InvalidType(Token, Token),
    #[error("Expected {0:?} but found {1:?}!")]
    InvalidTypeButFound(Token, Object),
    #[error("Found unknown identifier {0}!")]
    UnknownIdentifier(String),
    #[error("Could not convert {0} into {1}!")]
    CannotConvertInto(String, String),
    #[error("Cannot apply {0:?} on {1:#?}!")]
    IllegalPrefixOperation(Token, Box<Expression>),
    #[error("Cannot perform operation {0:?} on {1:?}!")]
    CannotApplyOn(Token, Box<Expression>),
    #[error("Cannot apply oepration {2:?} between {0:?} and {1:?}!")]
    IncompatibleTypes(Token, Object, Token)
}