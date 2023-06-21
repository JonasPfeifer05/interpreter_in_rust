use interpreter::lexer::token::Token;
use interpreter::parser::ast::expression::Expression;
use interpreter::parser::ast::statement::Statement;
use interpreter::parser::Parser;

#[test]
pub fn test_let_statement() {
    let tokens = vec![
        Token::Let,
        Token::Identifier("a".to_string()),
        Token::Assign,
        Token::Integer("10".to_string()),
        Token::Semicolon,
    ];

    let mut parser = Parser::from_tokens(tokens);

    assert_eq!(
        parser.parse_statement().unwrap(),
        Statement::Let {
            name: "a".to_string(),
            value: Box::new(Expression::Integer {
                value: "10".to_string()
            }),
        }
    )
}

#[test]
pub fn test_ret_statement() {
    let tokens = vec![
        Token::Return,
        Token::String("abc".to_string()),
        Token::Semicolon,
    ];

    let mut parser = Parser::from_tokens(tokens);

    assert_eq!(
        parser.parse_statement().unwrap(),
        Statement::Return {
            value: Box::new(Expression::String {
                value: "abc".to_string()
            }),
        }
    )
}

#[test]
pub fn test_expression_statement() {
    let tokens = vec![
        Token::Integer("1".to_string()),
        Token::Multiply,
        Token::Integer("2".to_string()),
        Token::Add,
        Token::Integer("3".to_string()),
        Token::Semicolon,
    ];

    let mut parser = Parser::from_tokens(tokens);

    assert_eq!(
        parser.parse_statement().unwrap(),
        Statement::Expression {
            value: Box::new(Expression::Infix {
                left: Box::new(Expression::Infix {
                    left: Box::new(Expression::Integer {
                        value: "1".to_string(),
                    }),
                    operation: Token::Multiply,
                    right: Box::new(Expression::Integer {
                        value: "2".to_string(),
                    }),
                }),
                operation: Token::Add,
                right: Box::new(Expression::Integer {
                    value: "3".to_string(),
                }),
            }),
        }
    )
}