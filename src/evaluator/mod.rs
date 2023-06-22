pub mod object;
pub mod environment;

use std::vec::IntoIter;
use clap::builder::EnumValueParser;
use crate::evaluator::environment::Environment;
use crate::evaluator::object::Object;
use crate::parser::ast::statement::Statement;

pub struct Evaluator {
    statements: IntoIter<Statement>,
}

impl Evaluator {
    pub fn new(statements: IntoIter<Statement>) -> Self {
        Self { statements }
    }
}

impl Evaluator {
    pub fn evaluate(&mut self, environment: &mut Environment) -> anyhow::Result<()> {
        while let Some(statement) = self.statements.next() {
            println!("{:?}", statement.evaluate(environment)?);
        }
        Ok(())
    }
}

pub fn evaluate_block(statements: &Vec<Box<Statement>>, remove_ret: bool, environment: &mut Environment) -> anyhow::Result<(Object, bool)> {
    let mut result = (Object::Null, false);
    for statement in statements {
        result = statement.evaluate(environment)?;
        if result.1 { break }
    }
    if remove_ret {
        Ok((result.0, false))
    } else {
        Ok(result)
    }
}
