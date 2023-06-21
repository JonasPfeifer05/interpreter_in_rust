use crate::repl::run_repl;

mod lexer;
mod parser;
mod evaluator;
mod error;
mod repl;

fn main() {
    run_repl();
}
