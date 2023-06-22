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
    }
}

impl Expression {
    pub fn evaluate(&self, environment: &mut Environment) -> anyhow::Result<(Object, bool)> {
        match self {
            Expression::Identifier { .. } => {}
            Expression::Integer { .. } => {}
            Expression::Float { .. } => {}
            Expression::String { .. } => {}
            Expression::Boolean { .. } => {}
            Expression::Prefix { .. } => {}
            Expression::Infix { .. } => {}
            Expression::If { .. } => {}
            Expression::While { .. } => {}
            Expression::Call { .. } => {}
            Expression::Error { .. } => {}
            Expression::Assign { .. } => {}
            Expression::Array { .. } => {}
            Expression::Block { .. } => {}
            Expression::Access { .. } => {}
        }
        todo!()
    }
}