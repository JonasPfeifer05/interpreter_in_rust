use thiserror::Error;
use crate::lexer::token::Token;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parser expected token but ran out of tokens!")]
    RanOutOfTokens,
    #[error("Parser expected {0} but found {1:?}!")]
    ExpectedButFound(String,Token),
    #[error("Parser expected {0:?} but found {1:?}!")]
    ExpectedTokenButFound(Token, Token),
    #[error("Found unexpected token: {0:?}!")]
    UnexpectedTokenFound(Token),
}