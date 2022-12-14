use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    input_bytes: &'a [u8],
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            input_bytes: input.as_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            b';' => Token::SEMICOLON,

            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::BANG
                }
            }
            b'/' => Token::SLASH,
            b'*' => Token::ASTERISK,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.read_identifier();
            }
            b'0'..=b'9' => {
                return self.read_number();
            }
            0 => Token::EOF,
            _ => Token::ILLEGAL,
        };

        self.read_char();
        tok
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input_bytes[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }
        self.input_bytes[self.read_position]
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
            "true" => Token::BOOL(true),
            "false" => Token::BOOL(false),
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,
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
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

1 == 1;
1 != 2;
"#;
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
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::BOOL(true),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::BOOL(false),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(1),
            Token::EQ,
            Token::INT(1),
            Token::SEMICOLON,
            Token::INT(1),
            Token::NotEq,
            Token::INT(2),
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
