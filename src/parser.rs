use crate::{
    ast::{Expression, Identifier, Infix, Literal, Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::ILLEGAL,
            peek_token: Token::ILLEGAL,
            errors: vec![],
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];

        while self.current_token != Token::EOF {
            match self.parse_statement() {
                Some(statement) => {
                    println!("statement parsed: {:?}", statement);
                    program.push(statement);
                }
                None => {
                    println!("statement not parsed")
                }
            }
            self.next_token();
        }

        program
    }

    pub fn get_errors(self) -> Vec<String> {
        self.errors
    }

    fn peek_error(&mut self, tok: Token) {
        let message = format!("next token: want={:?}, got={:?}", tok, self.peek_token);
        self.errors.push(message);
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::LET => self.parse_let_statement(),
            // Token::RETURN => self.parse_return_statement(),
            // Token::IF => self.parse_if_statement(),
            // TODO:
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        match self.peek_token {
            Token::IDENT(_) => self.next_token(), // skip LET
            _ => return None,
        };

        let name = match self.parse_identifier() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_peek(Token::ASSIGN) {
            return None;
        };

        while !self.current_token_is(Token::SEMICOLON) {
            // TODO: parse
            self.next_token();
        }
        // let expr = match self.parse_expression() {
        //     Some(expr) => expr,
        //     None => return None,
        // };
        // TODO: impl
        Some(Statement::Let(name, Expression::Literal(Literal::Int(5))))
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        match &self.current_token {
            Token::IDENT(ident) => Some(Identifier(ident.clone())),
            _ => None,
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        println!("parse_expression current_token: {:?}", self.current_token);
        match self.current_token {
            Token::INT(_) => match self.peek_token {
                Token::PLUS => self.parse_operator_expression(),
                Token::SEMICOLON => self.parse_integer_literal(),
                _ => None,
            },
            // Token::LPAREN => self.parse_grouped_expression(),
            _ => None,
        }
    }

    fn parse_operator_expression(&mut self) -> Option<Expression> {
        let lhs = match self.current_token {
            Token::INT(_) => match self.parse_integer_literal() {
                Some(expr) => expr,
                _ => return None,
            },
            _ => return None,
        };

        self.next_token();

        let op = match self.current_token {
            Token::PLUS => Infix::Plus,
            Token::MINUS => Infix::Minus,
            Token::ASTERISK => Infix::Multiply,
            Token::SLASH => Infix::Divide,
            Token::EQ => Infix::Eq,
            Token::NotEq => Infix::NotEq,
            Token::GT => Infix::GT,
            Token::LT => Infix::LT,
            _ => return None,
        };

        let rhs = match self.parse_expression() {
            Some(expr) => expr,
            _ => return None,
        };

        Some(Expression::Infix(op, Box::new(lhs), Box::new(rhs)))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        match self.current_token {
            Token::INT(val) => {
                self.next_token();
                Some(Expression::Literal(Literal::Int(val)))
            }
            _ => return None,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, tok: Token) -> bool {
        if self.peek_token_is(&tok) {
            self.next_token();
            true
        } else {
            self.peek_error(tok);
            false
        }
    }

    fn peek_token_is(&mut self, tok: &Token) -> bool {
        self.peek_token == *tok
    }

    fn current_token_is(&mut self, tok: Token) -> bool {
        self.current_token == tok
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        ast::{Expression, Identifier, Literal, Statement},
        lexer::Lexer,
    };

    use super::Parser;

    #[test]
    fn test_parse() {
        let mut parser = Parser::new(Lexer::new(
            r#"
let x = 5;
let y = 10;"#,
        ));

        let program = parser.parse();

        assert_eq!(
            program.get(0).unwrap().to_owned(),
            Statement::Let(
                Identifier("x".to_string()),
                Expression::Literal(Literal::Int(5))
            )
        );
        // assert_eq!(
        //     program.get(1).unwrap().to_owned(),
        //     Statement::Let(
        //         Identifier("y".to_string()),
        //         Expression::Literal(Literal::Int(10))
        //     )
        // );

        let mut parser = Parser::new(Lexer::new(r#"let x 5;"#));
        parser.parse();
        println!("{:?}", parser.errors); // "next token: want=ASSIGN, got=INT(5)"
        assert_eq!(parser.errors.len(), 1);
    }
}
