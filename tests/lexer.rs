use interpreter::lexer::{keyword_map, Lexer};
use interpreter::lexer::token::Token;

const LEXER_TEST_PATH: &'static str = "res/tests/lexer.txt";
const LEXER_TEST_STRING: &'static str = "abc 123 123.3 \"askdlk\" true false ; : , . ( ) { } [ ] = + - * / % ! && || == != > < >= <= function let if while int float string bool ret err null array";

#[test]
fn test_keywords() {
    let keywords = keyword_map();

    assert_eq!(keywords.len(), 15, "Keywords length do not match with the actual amount of keywords!")
}

#[test]
fn test_lexer_from_string() {
    let lexer = Lexer::from_string(LEXER_TEST_STRING.to_string());

    assert_eq!(lexer.program.collect::<Vec<char>>(), LEXER_TEST_STRING.chars().collect::<Vec<char>>(), "Lexer string and predefined string doesnt match!");
}

#[test]
fn test_lexer_from_path() {
    let lexer = Lexer::from_path(LEXER_TEST_PATH);

    assert!(lexer.is_ok(), "Error while creating lexer!");

    let lexer = lexer.unwrap();

    assert_eq!(lexer.program.collect::<Vec<char>>(), LEXER_TEST_STRING.chars().collect::<Vec<char>>());
}

#[test]
fn test_lexer_lex() {
    let tokens = vec![
        Token::Identifier("abc".to_string()),
        Token::Integer("123".to_string()),
        Token::Float("123.3".to_string()),
        Token::String("askdlk".to_string()),
        Token::Boolean("true".to_string()),
        Token::Boolean("false".to_string()),
        Token::Semicolon,
        Token::Colon,
        Token::Comma,
        Token::Dot,
        Token::LParent,
        Token::RParent,
        Token::LBrace,
        Token::RBrace,
        Token::LBracket,
        Token::RBracket,
        Token::Assign,
        Token::Add,
        Token::Subtract,
        Token::Multiply,
        Token::Divide,
        Token::Modular,
        Token::Invert,
        Token::And,
        Token::Or,
        Token::Equal,
        Token::NotEqual,
        Token::GreaterThan,
        Token::LessThan,
        Token::GreaterThanEqual,
        Token::LessThanEqual,
        Token::Function,
        Token::Let,
        Token::If,
        Token::While,
        Token::IntegerType,
        Token::FloatType,
        Token::StringType,
        Token::BooleanType,
        Token::Return,
        Token::Error,
        Token::NullType,
        Token::Array,
    ];

    let mut lexer = Lexer::from_path(LEXER_TEST_PATH).unwrap();

    assert_eq!(lexer.lex(), tokens, "Lexer generated the wrong tokens!")
}