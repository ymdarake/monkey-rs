use std::io;

use crate::{lexer::Lexer, token::Token};

pub fn start() {
    loop {
        println!(">> ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read_line");

        if input.trim().is_empty() {
            return;
        }

        let mut lexer = Lexer::new(&input);
        loop {
            match lexer.next_token() {
                Token::EOF => break,
                token => println!("{:?}", token),
            }
        }
    }
}
