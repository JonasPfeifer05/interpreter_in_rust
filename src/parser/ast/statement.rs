use anyhow::bail;
use crate::error::EvalError::{InvalidType, InvalidTypeButFound};
use crate::evaluator::environment::Environment;
use crate::evaluator::object::Object;
use crate::lexer::token::Token;
use crate::parser::ast::expression::Expression;

#[derive(Eq, PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
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
        typee: Token,
        body: Box<Expression>,
    },
}

impl Statement {
    pub fn evaluate(&self, environment: &mut Environment) -> anyhow::Result<(Object, bool)> {
        match self {
            Statement::Let {
                name,
                typee,
                value,
            } => {
                let (value, _) = value.evaluate(environment)?;
                match value {
                    Object::Integer(_) => if !typee.equal_variant(&Token::IntegerType) { bail!(InvalidType(typee.clone(), Token::IntegerType)) }
                    Object::Float(_) => if !typee.equal_variant(&Token::FloatType) { bail!(InvalidType(typee.clone(), Token::FloatType)) }
                    Object::String(_) => if !typee.equal_variant(&Token::StringType) { bail!(InvalidType(typee.clone(), Token::StringType)) }
                    Object::Boolean(_) => if !typee.equal_variant(&Token::BooleanType) { bail!(InvalidType(typee.clone(), Token::BooleanType)) }
                    Object::Array(_) => if !typee.equal_variant(&Token::ArrayType) { bail!(InvalidType(typee.clone(), Token::BooleanType)) }
                    Object::Error(_) => {}
                    obj => bail!(InvalidTypeButFound(typee.clone(), obj)),
                }
                environment.put(name.to_string(), value.clone());
                Ok((value, false))
            }
            Statement::Return {
                value
            } => {
                let (result, _) = value.evaluate(environment)?;
                Ok((result, true))
            }
            Statement::Expression {
                value,
            } => {
                value.evaluate(environment)
            }
            Statement::Function {
                name,
                parameter,
                typee,
                body,
            } => {
                environment.put(name.clone(), Object::Function {
                   typee: typee.clone(),
                    parameters: parameter.clone(),
                    body: body.clone()
                });
                Ok((Object::Null, false))
            }
        }
    }
}