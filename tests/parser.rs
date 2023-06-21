use interpreter::lexer::token::Token;
use interpreter::parser::ast::expression::Expression;
use interpreter::parser::ast::statement::Statement;
use interpreter::parser::Parser;
use interpreter::parser::precedences::Precedences;

#[test]
fn test_prefix_expression() {
    let tokens = vec![
        Token::Subtract,
        Token::Integer("123".to_string()),
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_expression(Precedences::Lowest).unwrap(),
        Expression::Prefix {
            prefix: Token::Subtract,
            value: Box::new(Expression::Integer {
                value: "123".to_string()
            }),
        }
    )
}

#[test]
fn test_infix_expression() {
    let tokens = vec![
        Token::Integer("123".to_string()),
        Token::Add,
        Token::Integer("123".to_string()),
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_expression(Precedences::Lowest).unwrap(),
        Expression::Infix {
            left: Box::new(Expression::Integer {
                value: "123".to_string(),
            }),
            operation: Token::Add,
            right: Box::new(Expression::Integer {
                value: "123".to_string()
            }),
        }
    )
}

#[test]
fn test_if_expression() {
    let tokens = vec![
        Token::If,
        Token::LParent,
        Token::Boolean("true".to_string()),
        Token::RParent,
        Token::LBrace,
        Token::Integer("1".to_string()),
        Token::Semicolon,
        Token::RBrace,
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_expression(Precedences::Lowest).unwrap(),
        Expression::If {
            condition: Box::new(Expression::Boolean {
                value: "true".to_string()
            }),
            consequence: Box::new(Expression::Block {
                statements: vec![Box::new(Statement::Expression {
                    value: Box::new(Expression::Integer {
                        value: "1".to_string()
                    })
                })]
            }),
            alternative: None,
        }
    )
}

#[test]
pub fn test_if_else_expression() {
    let tokens = vec![
        Token::If,
        Token::LParent,
        Token::Boolean("true".to_string()),
        Token::RParent,
        Token::LBrace,
        Token::Integer("1".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::Integer("1".to_string()),
        Token::Semicolon,
        Token::RBrace,
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_expression(Precedences::Lowest).unwrap(),
        Expression::If {
            condition: Box::new(Expression::Boolean {
                value: "true".to_string()
            }),
            consequence: Box::new(Expression::Block {
                statements: vec![Box::new(Statement::Expression {
                    value: Box::new(Expression::Integer {
                        value: "1".to_string()
                    })
                })]
            }),
            alternative: Some(Box::new(Expression::Block {
                statements: vec![Box::new(Statement::Expression {
                    value: Box::new(Expression::Integer {
                        value: "1".to_string()
                    })
                })]
            })),
        }
    )
}

#[test]
fn test_while_expression() {
    let tokens = vec![
        Token::While,
        Token::LParent,
        Token::Boolean("true".to_string()),
        Token::RParent,
        Token::LBrace,
        Token::Integer("1".to_string()),
        Token::Semicolon,
        Token::RBrace,
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_expression(Precedences::Lowest).unwrap(),
        Expression::While {
            condition: Box::new(Expression::Boolean {
                value: "true".to_string()
            }),
            consequence: Box::new(Expression::Block {
                statements: vec![Box::new(Statement::Expression {
                    value: Box::new(Expression::Integer {
                        value: "1".to_string()
                    })
                })]
            }),
        }
    )
}

#[test]
fn test_call_expression() {
    let tokens = vec![
        Token::Identifier("call".to_string()),
        Token::LParent,
        Token::Integer("1".to_string()),
        Token::RParent,
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_expression(Precedences::Lowest).unwrap(),
        Expression::Call {
            name: "call".to_string(),
            arguments: vec![Box::new(Expression::Integer {
                value: "1".to_string()
            })],
        }
    )
}

#[test]
fn test_error_expression() {
    let tokens = vec![
        Token::Error,
        Token::LParent,
        Token::Integer("1".to_string()),
        Token::RParent,
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_expression(Precedences::Lowest).unwrap(),
        Expression::Error {
            value: Box::new(Expression::Integer {
                value: "1".to_string()
            })
        }
    )
}

#[test]
fn test_assign_expression() {
    let tokens = vec![
        Token::Identifier("a".to_string()),
        Token::Assign,
        Token::Integer("1".to_string()),
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_expression(Precedences::Lowest).unwrap(),
        Expression::Assign {
            name: "a".to_string(),
            value: Box::new(Expression::Integer {
                value: "1".to_string()
            }),
        }
    )
}

#[test]
fn test_array_expression() {
    let tokens = vec![
        Token::LBracket,
        Token::Integer("1".to_string()),
        Token::Comma,
        Token::Integer("2".to_string()),
        Token::RBracket,
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_expression(Precedences::Lowest).unwrap(),
        Expression::Array {
            values: vec![
                Box::new(Expression::Integer {
                    value: "1".to_string(),
                }),
                Box::new(Expression::Integer {
                    value: "2".to_string()
                }),
            ]
        }
    )
}

#[test]
fn test_block_expression() {
    let tokens = vec![
        Token::LBrace,
        Token::Integer("1".to_string()),
        Token::Semicolon,
        Token::Integer("2".to_string()),
        Token::Semicolon,
        Token::RBrace,
    ];

    let mut parser = Parser::from_tokens(tokens);
    assert_eq!(
        parser.parse_block_expression().unwrap(),
        Expression::Block {
            statements: vec![
                Box::new(Statement::Expression {
                    value: Box::new(Expression::Integer {
                        value: "1".to_string(),
                    })
                }),
                Box::new(Statement::Expression {
                    value: Box::new(Expression::Integer {
                        value: "2".to_string(),
                    })
                }),
            ]
        }
    )
}

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

#[test]
pub fn test_function_statement() {
    let tokens = vec![
        Token::Function,
        Token::Identifier("test".to_string()),
        Token::LParent,
        Token::Identifier("a".to_string()),
        Token::Colon,
        Token::IntegerType,
        Token::RParent,
        Token::LBrace,
        Token::Integer("1".to_string()),
        Token::Semicolon,
        Token::RBrace,
    ];

    let mut parser = Parser::from_tokens(tokens);

    assert_eq!(
        parser.parse_statement().unwrap(),
        Statement::Function {
            name: "test".to_string(),
            parameter: vec![("a".to_string(), Token::IntegerType)],
            body: Box::new(Expression::Block {
                statements: vec![Box::new(Statement::Expression {
                    value: Box::new(Expression::Integer {
                        value: "1".to_string(),
                    })
                })],
            }),
        }
    )
}