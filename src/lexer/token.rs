use derive_more::Constructor;

use super::operator::Operator;

#[derive(Debug, Constructor)]
pub struct Token {
    pub value: TokenValue,
    pub index: usize,
}

#[derive(Debug, PartialEq)]
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
