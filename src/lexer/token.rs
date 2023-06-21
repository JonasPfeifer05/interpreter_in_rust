pub enum Token {
    // Types
    Identifier(String),
    Integer(String),
    Float(String),
    String(String),
    Boolean(String),

    // Special Characters
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
    Sub,
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

    IntegerType,
    FloatType,
    StringType,
    BoolType,
}