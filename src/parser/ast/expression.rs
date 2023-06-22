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
                Ok((evaluate_infix_expression(left, left.evaluate(environment)?.0, right.evaluate(environment)?.0, operation)?, false))
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

pub fn evaluate_infix_expression(
    left: &Box<Expression>,
    left_obj: Object,
    right_obj: Object,
    operation: &Token,
) -> anyhow::Result<Object> {
    let (
        integer_op,
        float_op,
        string_op,
        bool_op,
        int_float_mixable,
    ) = get_apply_functions(operation);

    Ok(match left_obj {
        Object::Integer(left_val) if integer_op.is_some() => match right_obj {
            Object::Integer(right_val) => integer_op.unwrap()(left_val, right_val),
            Object::Float(right_val) if int_float_mixable => float_op.unwrap()(left_val as f64, right_val),
            obj => bail!(IncompatibleTypes(Token::IntegerType, obj, operation.clone()))
        }
        Object::Float(left_val) if float_op.is_some() => match right_obj {
            Object::Integer(right_val) => float_op.unwrap()(left_val, right_val as f64),
            Object::Float(right_val) if int_float_mixable => float_op.unwrap()(left_val, right_val),
            obj => bail!(IncompatibleTypes(Token::FloatType, obj, operation.clone()))
        }
        Object::String(left_val) if string_op.is_some() => match right_obj {
            Object::String(right_val) => string_op.unwrap()(left_val, right_val),
            obj => bail!(IncompatibleTypes(Token::StringType, obj, operation.clone()))
        }
        Object::Boolean(left_val) if bool_op.is_some() => match right_obj {
            Object::Boolean(right_val) => bool_op.unwrap()(left_val,right_val),
            obj => bail!(IncompatibleTypes(Token::BooleanType, obj, operation.clone()))
        }
        _ => bail!(CannotApplyOn(operation.clone(), left.clone()))
    })
}

pub fn get_apply_functions(operator: &Token) -> (
    Option<fn(i128, i128) -> Object>,
    Option<fn(f64, f64) -> Object>,
    Option<fn(String, String) -> Object>,
    Option<fn(bool, bool) -> Object>,
    bool,
) {
    match operator {
        Token::Add => (
            Some(|a: i128, b: i128| Object::Integer(a + b)),
            Some(|a: f64, b: f64| Object::Float(a + b)),
            Some(|a: String, b: String| Object::String(format!("{}{}", a, b))),
            None,
            true,
        ),
        Token::Subtract => (
            Some(|a: i128, b: i128| Object::Integer(a - b)),
            Some(|a: f64, b: f64| Object::Float(a - b)),
            None,
            None,
            true,
        ),
        Token::Multiply => (
            Some(|a: i128, b: i128| Object::Integer(a * b)),
            Some(|a: f64, b: f64| Object::Float(a * b)),
            None,
            None,
            true,
        ),
        Token::Divide => (
            Some(|a: i128, b: i128| Object::Integer(a / b)),
            Some(|a: f64, b: f64| Object::Float(a / b)),
            None,
            None,
            true,
        ),
        Token::Modular => (
            Some(|a: i128, b: i128| Object::Integer(a % b)),
            Some(|a: f64, b: f64| Object::Float(a % b)),
            None,
            None,
            true,
        ),
        Token::And => (
            None,
            None,
            None,
            Some(|a: bool,b: bool| Object::Boolean(a && b)),
            false,
        ),
        Token::Or => (
            None,
            None,
            None,
            Some(|a: bool,b: bool| Object::Boolean(a || b)),
            false
        ),
        Token::Equal => (
            Some(|a: i128,b: i128| Object::Boolean(a == b)),
            Some(|a: f64,b: f64| Object::Boolean(a == b)),
            Some(|a: String,b: String| Object::Boolean(a == b)),
            Some(|a: bool,b: bool| Object::Boolean(a == b)),
            false,
        ),
        Token::NotEqual => (
            Some(|a: i128,b: i128| Object::Boolean(a != b)),
            Some(|a: f64,b: f64| Object::Boolean(a != b)),
            Some(|a: String,b: String| Object::Boolean(a != b)),
            Some(|a: bool,b: bool| Object::Boolean(a != b)),
            false,
        ),
        Token::GreaterThan => (
            Some(|a: i128,b: i128| Object::Boolean(a > b)),
            Some(|a: f64,b: f64| Object::Boolean(a > b)),
            None,
            None,
            true,
        ),
        Token::LessThan => (
            Some(|a: i128,b: i128| Object::Boolean(a < b)),
            Some(|a: f64,b: f64| Object::Boolean(a < b)),
            None,
            None,
            true,
        ),
        Token::GreaterThanEqual => (
            Some(|a: i128,b: i128| Object::Boolean(a >= b)),
            Some(|a: f64,b: f64| Object::Boolean(a >= b)),
            None,
            None,
            true,
        ),
        Token::LessThanEqual => (
            Some(|a: i128,b: i128| Object::Boolean(a <= b)),
            Some(|a: f64,b: f64| Object::Boolean(a <= b)),
            None,
            None,
            true,
        ),
        _ => unreachable!(),
    }
}