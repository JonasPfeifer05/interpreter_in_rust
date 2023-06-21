use std::io::{stdin, stdout, Write};
use std::time::Instant;
use colored::Colorize;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn run_repl() {
    println!("{}", "Welcome to p_lang! Just enter your commands: ".bright_green());
    loop {
        let program = read_from_stdin(">> ");
        if &program == &"exit" { break }

        let start = Instant::now();
        let mut lexer = Lexer::from_string(program);
        let tokens = lexer.lex();
        println!("{}", format!("{:?}", tokens).bright_blue());
        println!("{}", format!("Lexing took {:?}", start.elapsed()).magenta());

        let start = Instant::now();
        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse();
        if let Ok(ast) = ast {
            println!("{}", format!("{:#?}", ast).bright_blue());
            println!("{}", format!("Parsing took {:?}", start.elapsed()).magenta());
        } else if let Err(err) = ast {
            eprintln!("{}", format!("{}", err).bright_red());
            println!("{}", format!("Parsing took {:?}", start.elapsed()).magenta());
            continue
        }
    }
}

pub fn read_from_stdin(prompt: &str) -> String {
    let mut buf = String::new();
    print!("{prompt}");
    stdout().lock().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}