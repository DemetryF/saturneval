use derive_more::Constructor;

use super::operator::Operator;

#[derive(Debug, Constructor, Clone)]
pub struct Token {
    pub value: TokenValue,
    pub index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Number(String),
    Id(String),

    Eof,

    // special chars
    Comma,
    OpeningParen,
    ClosingParen,

    Operator(Operator),
}
