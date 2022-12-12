use repl::start;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    println!("Hello there! This is the Monkey programming language!\n");
    start();
}
