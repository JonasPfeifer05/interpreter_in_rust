use anyhow::bail;
use crate::error::EvalError::{CannotApplyOn, CannotConvertInto, IllegalPrefixOperation, IncompatibleTypes, UnknownIdentifier};
use crate::evaluator::environment::Environment;
use crate::evaluator::object::Object;
use crate::lexer::token::Token;
use crate::parser::ast::statement::Statement;

#[derive(Eq, PartialEq)]
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier {
        name: String,
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
        prefix: Token,
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
        statements: Vec<Box<Statement>>
    },
    Access {
        source: Box<Expression>,
        index: Box<Expression>,
    },
}

impl Expression {
    pub fn evaluate(&self, environment: &mut Environment) -> anyhow::Result<(Object, bool)> {
        match self {
            Expression::Identifier { name } => {
                if let Some(value) = environment.search(name) {
                    Ok((value.clone(), false))
                } else {
                    bail!(UnknownIdentifier(name.to_string()))
                }
            }
            Expression::Integer { value } => {
                if let Ok(value) = value.parse::<i128>() {
                    Ok((Object::Integer(value), false))
                } else {
                    bail!(CannotConvertInto(value.clone(), "Integer".to_string()))
                }
            }
            Expression::Float { value } => {
                if let Ok(value) = value.parse::<f64>() {
                    Ok((Object::Float(value), false))
                } else {
                    bail!(CannotConvertInto(value.clone(), "Float".to_string()))
                }
            }
            Expression::String { value } => {
                Ok((Object::String(value.clone()), false))
            }
            Expression::Boolean { value } => {
                if value == "true" {
                    Ok((Object::Boolean(true), false))
                } else {
                    Ok((Object::Boolean(false), false))
                }
            }
            Expression::Prefix { prefix, value } => {
                let (object, _) = value.evaluate(environment)?;

                let object = match prefix {
                    Token::Subtract => {
                        match object {
                            Object::Integer(val) => Object::Integer(-val),
                            Object::Float(val) => Object::Float(-val),
                            _ => bail!(IllegalPrefixOperation(Token::Subtract, value.clone()))
                        }
                    }
                    Token::Invert => {
                        match object {
                            Object::Boolean(val) => Object::Boolean(!val),
                            _ => bail!(IllegalPrefixOperation(Token::Invert, value.clone())),
                        }
                    }
                    _ => unreachable!(),
                };

                Ok((object, false))
            }
            Expression::Infix { left, operation, right } => {
                match operation {
                    Token::Add => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Integer(left + right), false)),
                            Object::Float(right) => Ok((Object::Float(left as f64 + right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::Add))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Float(left + right as f64), false)),
                            Object::Float(right) => Ok((Object::Float(left + right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::Add))
                        }
                        _ => bail!(CannotApplyOn(Token::Add, left.clone()))
                    }
                    Token::Subtract => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Integer(left - right), false)),
                            Object::Float(right) => Ok((Object::Float(left as f64 - right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::Subtract))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Float(left - right as f64), false)),
                            Object::Float(right) => Ok((Object::Float(left - right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::Subtract))
                        }
                        _ => bail!(CannotApplyOn(Token::Subtract, left.clone()))
                    }
                    Token::Multiply => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Integer(left * right), false)),
                            Object::Float(right) => Ok((Object::Float(left as f64 * right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::Multiply))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Float(left * right as f64), false)),
                            Object::Float(right) => Ok((Object::Float(left * right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::Multiply))
                        }
                        _ => bail!(CannotApplyOn(Token::Multiply, left.clone()))
                    }
                    Token::Divide => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Integer(left / right), false)),
                            Object::Float(right) => Ok((Object::Float(left as f64 / right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::Divide))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Float(left / right as f64), false)),
                            Object::Float(right) => Ok((Object::Float(left / right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::Divide))
                        }
                        _ => bail!(CannotApplyOn(Token::Divide, left.clone()))
                    }
                    Token::Modular => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Integer(left % right), false)),
                            Object::Float(right) => Ok((Object::Float(left as f64 % right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::Modular))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Float(left % right as f64), false)),
                            Object::Float(right) => Ok((Object::Float(left % right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::Modular))
                        }
                        _ => bail!(CannotApplyOn(Token::Modular, left.clone()))
                    }
                    Token::And => match left.evaluate(environment)?.0 {
                        Object::Boolean(left) => match right.evaluate(environment)?.0 {
                            Object::Boolean(right) => Ok((Object::Boolean(left && right), false)),
                            obj => bail!(IncompatibleTypes(Token::BooleanType, obj, Token::And))
                        }
                        _ => bail!(CannotApplyOn(Token::And, left.clone()))
                    }
                    Token::Or => match left.evaluate(environment)?.0 {
                        Object::Boolean(left) => match right.evaluate(environment)?.0 {
                            Object::Boolean(right) => Ok((Object::Boolean(left || right), false)),
                            obj => bail!(IncompatibleTypes(Token::BooleanType, obj, Token::Or))
                        }
                        _ => bail!(CannotApplyOn(Token::And, left.clone()))
                    }
                    Token::Equal => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left == right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::Equal))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Float(right) => Ok((Object::Boolean(left == right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::Equal))
                        }
                        Object::Boolean(left) => match right.evaluate(environment)?.0 {
                            Object::Boolean(right) => Ok((Object::Boolean(left == right), false)),
                            obj => bail!(IncompatibleTypes(Token::BooleanType, obj, Token::Equal))
                        }
                        _ => bail!(CannotApplyOn(Token::And, left.clone()))
                    }
                    Token::NotEqual => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left != right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::Equal))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Float(right) => Ok((Object::Boolean(left != right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::Equal))
                        }
                        Object::Boolean(left) => match right.evaluate(environment)?.0 {
                            Object::Boolean(right) => Ok((Object::Boolean(left != right), false)),
                            obj => bail!(IncompatibleTypes(Token::BooleanType, obj, Token::Equal))
                        }
                        _ => bail!(CannotApplyOn(Token::NotEqual, left.clone()))
                    }
                    Token::GreaterThan => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left > right), false)),
                            Object::Float(right) => Ok((Object::Boolean(left as f64 > right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::GreaterThan))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left > right as f64), false)),
                            Object::Float(right) => Ok((Object::Boolean(left > right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::GreaterThan))
                        }
                        _ => bail!(CannotApplyOn(Token::Modular, left.clone()))
                    }
                    Token::LessThan => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left < right), false)),
                            Object::Float(right) => Ok((Object::Boolean((left as f64) < right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::LessThan))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left < right as f64), false)),
                            Object::Float(right) => Ok((Object::Boolean(left < right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::LessThan))
                        }
                        _ => bail!(CannotApplyOn(Token::Modular, left.clone()))
                    }
                    Token::GreaterThanEqual => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left >= right), false)),
                            Object::Float(right) => Ok((Object::Boolean(left as f64 >= right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::GreaterThanEqual))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left >= right as f64), false)),
                            Object::Float(right) => Ok((Object::Boolean(left >= right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::GreaterThanEqual))
                        }
                        _ => bail!(CannotApplyOn(Token::Modular, left.clone()))
                    }
                    Token::LessThanEqual => match left.evaluate(environment)?.0 {
                        Object::Integer(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left <= right), false)),
                            Object::Float(right) => Ok((Object::Boolean(left as f64 <= right), false)),
                            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, Token::LessThanEqual))
                        }
                        Object::Float(left) => match right.evaluate(environment)?.0 {
                            Object::Integer(right) => Ok((Object::Boolean(left <= right as f64), false)),
                            Object::Float(right) => Ok((Object::Boolean(left <= right), false)),
                            obj => bail!(IncompatibleTypes(Token::FloatType, obj, Token::LessThanEqual))
                        }
                        _ => bail!(CannotApplyOn(Token::Modular, left.clone()))
                    }
                    _ => unreachable!(),
                }
            }
            Expression::If { condition, consequence, alternative } => { todo!() }
            Expression::While { condition, consequence } => { todo!() }
            Expression::Call { name, arguments } => { todo!() }
            Expression::Error { value } => { todo!() }
            Expression::Assign { name, value } => { todo!() }
            Expression::Array { values } => { todo!() }
            Expression::Block { statements } => { todo!() }
            Expression::Access { source, index } => { todo!() }
        }
    }
}