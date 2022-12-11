use repl::start;

pub mod lexer;
pub mod repl;
pub mod token;

fn main() {
    println!("Hello there! This is the Monkey programming language!\n");
    start();
}
