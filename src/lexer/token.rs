/// A token represents a single part of a statement inside the programming language
#[derive(Eq, PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Token {
    // Represents an unknown part inside a program
    Illegal,

    // Types
    Identifier(String),
    Integer(String),
    Float(String),
    String(String),
    Boolean(String),

    // Special characters
    Semicolon,
    Colon,
    Comma,
    Dot,

    // Brackets
    LParent,
    RParent,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Operators
    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modular,

    Invert,
    And,
    Or,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,

    // Keywords
    Function,
    Let,

    If,
    While,

    IntegerType,
    FloatType,
    StringType,
    BooleanType,
}