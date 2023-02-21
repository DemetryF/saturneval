use derive_more::Constructor;

use crate::lexer::Operator;

#[derive(Debug)]
pub enum Expr {
    Prefix(Prefix),
    Infix(Infix),
    Call(Call),
    Atom(Atom),
}

#[derive(Debug, Constructor)]
pub struct Prefix {
    pub op: Operator,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Constructor)]
pub struct Infix {
    pub lhs: Box<Expr>,
    pub op: Operator,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Constructor)]
pub struct Call {
    pub id: Id,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub enum Atom {
    Number(String),
    Id(Id),
}

#[derive(Debug, Constructor)]
pub struct Id {
    pub value: String,
    pub index: usize,
}
