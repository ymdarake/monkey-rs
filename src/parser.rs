use crate::{
    ast::{Expression, Identifier, Infix, Literal, Precedence, Prefix, Program, Statement},
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
            Token::RETURN => self.parse_return_statement(),
            // Token::IF => self.parse_if_statement(),
            // TODO:
            _ => self.parse_expression_statement(),
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

    fn parse_return_statement(&mut self) -> Option<Statement> {
        if !self.current_token_is(Token::RETURN) {
            return None;
        }

        self.next_token(); // skip return

        // TODO: impl
        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }
        Some(Statement::Return(Expression::Literal(Literal::Int(1))))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token_is(&Token::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::Expression(expression))
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        match &self.current_token {
            Token::IDENT(ident) => Some(Identifier(ident.clone())),
            _ => None,
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // parse prefix expression
        let mut left = match self.current_token {
            Token::IDENT(_) => self.parse_identifier_expression(),
            Token::INT(_) => self.parse_integer_literal_expression(),
            Token::BANG | Token::PLUS | Token::MINUS => self.parse_prefix_expression(),
            // Token::STRING(_) => self.parse_string_expression(),
            _ => {
                self.errors.push(format!(
                    "no prefix parser function for {:?} found",
                    self.current_token
                ));
                return None;
            }
        };

        // parse infix expression
        while !self.peek_token_is(&Token::SEMICOLON) && precedence < self.peek_precedence() {
            match self.peek_token {
                Token::PLUS
                | Token::MINUS
                | Token::SLASH
                | Token::ASTERISK
                | Token::EQ
                | Token::NotEq
                | Token::LT
                | Token::GT => {
                    self.next_token();
                    left = self.parse_infix_expression(left.unwrap());
                }
                // Token::LPAREN => {
                //     self.next_token();
                //     left = self.parse_call_expression(left.unwrap());
                // }
                _ => return left,
            }
        }

        left
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let infix = match self.current_token {
            Token::PLUS => Infix::Plus,
            Token::MINUS => Infix::Minus,
            Token::SLASH => Infix::Divide,
            Token::ASTERISK => Infix::Multiply,
            Token::EQ => Infix::Eq,
            Token::NotEq => Infix::NotEq,
            Token::LT => Infix::LT,
            Token::GT => Infix::GT,
            _ => return None,
        };

        let precedence = self.token_to_precedence(&self.current_token.clone());
        self.next_token();
        match self.parse_expression(precedence) {
            Some(expr) => Some(Expression::Infix(infix, Box::new(left), Box::new(expr))),
            None => None,
        }
    }

    fn parse_identifier_expression(&mut self) -> Option<Expression> {
        match self.parse_identifier() {
            Some(ident) => Some(Expression::Identifier(ident)),
            _ => None,
        }
    }

    fn parse_integer_literal_expression(&mut self) -> Option<Expression> {
        match self.current_token {
            Token::INT(val) => Some(Expression::Literal(Literal::Int(val))),
            _ => return None,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let prefix = match self.current_token {
            Token::BANG => Prefix::Not,
            Token::PLUS => Prefix::Plus,
            Token::MINUS => Prefix::Minus,
            _ => return None,
        };

        self.next_token();

        match self.parse_expression(Precedence::Prefix) {
            Some(expr) => Some(Expression::Prefix(prefix, Box::new(expr))),
            None => None,
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

    fn peek_precedence(&mut self) -> Precedence {
        self.token_to_precedence(&self.peek_token.clone())
    }

    fn token_to_precedence(&mut self, tok: &Token) -> Precedence {
        match tok {
            Token::EQ | Token::NotEq => Precedence::Equals,
            Token::LT | Token::GT => Precedence::LessGreater,
            Token::PLUS | Token::MINUS => Precedence::Sum,
            Token::SLASH | Token::ASTERISK => Precedence::Product,
            Token::LPAREN => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn current_token_is(&mut self, tok: Token) -> bool {
        self.current_token == tok
    }
}

#[cfg(test)]
mod tests {

    use std::borrow::Borrow;

    use crate::{
        ast::{Expression, Identifier, Infix, Literal, Prefix, Statement},
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

    #[test]
    fn test_return_statements() {
        let mut parser = Parser::new(Lexer::new(
            r#"
return 5;
return 10;
return 993322;"#,
        ));

        parser.parse();
        println!("{:?}", parser.errors);
    }

    #[test]
    fn test_prefix() {
        let mut parser = Parser::new(Lexer::new(
            r#"
!5;
-15;"#,
        ));
        let program = parser.parse();

        assert_eq!(
            program.get(0).unwrap().to_owned(),
            Statement::Expression(Expression::Prefix(
                Prefix::Not,
                Box::new(Expression::Literal(Literal::Int(5)))
            ))
        );
        assert_eq!(
            program.get(1).unwrap().to_owned(),
            Statement::Expression(Expression::Prefix(
                Prefix::Minus,
                Box::new(Expression::Literal(Literal::Int(15)))
            ))
        );
    }

    #[test]
    fn test_infix() {
        let mut parser = Parser::new(Lexer::new(
            r#"
5 + 5;
5 - 5;
5 * 5;
5 / 5;
5 > 5;
5 < 5;
5 == 5;
5 != 5;"#,
        ));
        let program = parser.parse();
        let expects = vec![
            Infix::Plus,
            Infix::Minus,
            Infix::Multiply,
            Infix::Divide,
            Infix::GT,
            Infix::LT,
            Infix::Eq,
            Infix::NotEq,
        ];

        for (i, infix) in expects.iter().enumerate() {
            assert_eq!(
                program.get(i).unwrap().to_owned(),
                Statement::Expression(Expression::Infix(
                    infix.to_owned(),
                    Box::new(Expression::Literal(Literal::Int(5))),
                    Box::new(Expression::Literal(Literal::Int(5))),
                ))
            );
        }
    }

    #[test]
    fn test_precedence() {
        let mut parser = Parser::new(Lexer::new(
            r#"
-a * b;
!-a;
a + b + c;"#,
        ));
        let program = parser.parse();

        assert_eq!(
            program.get(0).unwrap().to_owned(),
            Statement::Expression(Expression::Infix(
                Infix::Multiply,
                Box::new(Expression::Prefix(
                    Prefix::Minus,
                    Box::new(Expression::Identifier(Identifier(String::from("a"))))
                )),
                Box::new(Expression::Identifier(Identifier(String::from("b"))))
            ))
        );
        assert_eq!(
            program.get(1).unwrap().to_owned(),
            Statement::Expression(Expression::Prefix(
                Prefix::Not,
                Box::new(Expression::Prefix(
                    Prefix::Minus,
                    Box::new(Expression::Identifier(Identifier(String::from("a"))))
                ),),
            ))
        );

        assert_eq!(
            program.get(2).unwrap().to_owned(),
            Statement::Expression(Expression::Infix(
                Infix::Plus,
                Box::new(Expression::Infix(
                    Infix::Plus,
                    Box::new(Expression::Identifier(Identifier(String::from("a")))),
                    Box::new(Expression::Identifier(Identifier(String::from("b")))),
                )),
                Box::new(Expression::Identifier(Identifier(String::from("c"))))
            ))
        );
    }
}
