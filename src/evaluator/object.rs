use crate::error::EvalError::UnknownIdentifier;
use crate::evaluator::environment::Environment;
use crate::lexer::token::Token;
use crate::parser::ast::expression::Expression;

#[derive(Debug, Clone)]
pub enum OwnerShip {
    Reference(String),
    Instance(Object),
}

impl OwnerShip {
    pub fn value(&self, environment: &Environment) -> anyhow::Result<Object> {
        match self {
            OwnerShip::Reference(identifier) => {
                let mut value = environment.get(identifier).ok_or(UnknownIdentifier(identifier.clone()))?.clone();

                loop {
                    match value {
                        OwnerShip::Reference(identifier) => {
                            value = environment.get(&identifier).ok_or(UnknownIdentifier(identifier))?.clone();
                        }
                        OwnerShip::Instance(val) => {
                            return Ok(val);
                        }
                    }
                }
            },
            OwnerShip::Instance(val) => Ok(val.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i128),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<OwnerShip>),
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