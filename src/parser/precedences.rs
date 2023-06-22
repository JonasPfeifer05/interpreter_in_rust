use crate::lexer::token::Token;

#[repr(u8)]
#[derive(Ord, PartialOrd)]
#[derive(Eq, PartialEq)]
pub enum Precedences {
    Lowest,
    OrAnd,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Postfix,
    Call,
    Assign,
}

impl Token {
    pub fn precedence(&self) -> Precedences {
        match self {
            Token::Add |
            Token::Subtract => Precedences::Sum,
            Token::Or |
            Token::And => Precedences::OrAnd,

            Token::Multiply |
            Token::Divide |
            Token::Modular => Precedences::Product,

            Token::Equal |
            Token::NotEqual => Precedences::Equals,

            Token::LessThan |
            Token::GreaterThan |
            Token::LessThanEqual |
            Token::GreaterThanEqual => Precedences::LessGreater,

            Token::Invert => Precedences::Prefix,

            Token::LParent => Precedences::Call,

            Token::Assign => Precedences::Assign,

            Token::LBracket => Precedences::Postfix,

            _ => Precedences::Lowest
        }
    }
}