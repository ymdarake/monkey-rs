#[derive(PartialEq, Clone, Debug)]
pub struct Identifier(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Eq,
    NotEq,
    GTE,
    GT,
    LTE,
    LT,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Infix(Infix, Box<Expression>, Box<Expression>),
}

#[derive(PartialEq, Clone, Debug)]

pub enum Literal {
    Int(i64),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Let(Identifier, Expression),
}

pub type Program = Vec<Statement>;
