use std::io::{stdin, stdout, Write};
use colored::Colorize;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn run_repl() {
    println!("{}", "Welcome to p_lang! Just enter your commands: ".bright_green());
    loop {
        let program = read_from_stdin(">> ");
        if &program == &"exit" { break }

        let mut lexer = Lexer::from_string(program);
        let tokens = lexer.lex();
        println!("{}", format!("{:?}", tokens).bright_blue());

        let mut parser = Parser::from_tokens(tokens);
        let result = parser.parse();
        println!("{:#?}", result);
    }
}

pub fn read_from_stdin(prompt: &str) -> String {
    let mut buf = String::new();
    print!("{prompt}");
    stdout().lock().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}