use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.ch {
            b'=' => {
                self.read_char();
                Token::ASSIGN
            }
            b';' => {
                self.read_char();
                Token::SEMICOLON
            }
            b'(' => {
                self.read_char();
                Token::LPAREN
            }
            b')' => {
                self.read_char();
                Token::RPAREN
            }
            b',' => {
                self.read_char();
                Token::COMMA
            }
            b'+' => {
                self.read_char();
                Token::PLUS
            }
            b'-' => {
                self.read_char();
                Token::MINUS
            }
            b'!' => {
                self.read_char();
                Token::BANG
            }
            b'/' => {
                self.read_char();
                Token::SLASH
            }
            b'*' => {
                self.read_char();
                Token::ASTERISK
            }
            b'<' => {
                self.read_char();
                Token::LT
            }
            b'>' => {
                self.read_char();
                Token::GT
            }
            b'{' => {
                self.read_char();
                Token::LBRACE
            }
            b'}' => {
                self.read_char();
                Token::RBRACE
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_identifier(),
            b'0'..=b'9' => self.read_number(),
            0 => Token::EOF,
            _ => Token::ILLEGAL,
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            // TODO: optimize
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> Token {
        let start_position = self.position;
        loop {
            match self.ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.read_char();
                }
                _ => break,
            }
        }

        let literal = &self.input[start_position..self.position];
        match literal {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            _ => Token::IDENT(String::from(literal)),
        }
    }

    fn read_number(&mut self) -> Token {
        let start_position = self.position;
        loop {
            match self.ch {
                b'0'..=b'9' => {
                    self.read_char();
                }
                _ => break,
            }
        }
        let literal = &self.input[start_position..self.position];

        Token::INT(literal.parse::<i64>().unwrap())
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{lexer, token::Token};

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;"#;
        let tests = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut lexer = lexer::Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();
            assert_eq!(expect, tok);
        }
    }
}
