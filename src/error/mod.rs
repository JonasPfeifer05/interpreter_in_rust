use thiserror::Error;
use crate::lexer::token::Token;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parser expected {0} but found {1:?}!")]
    ExpectedButFound(String,Token)
}