use std::fmt::Display;

use derive_more::Constructor;

use super::Operator;

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

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::Id(id) => write!(f, "{id}"),
            Self::Eof => write!(f, "eof"),
            Self::Comma => write!(f, ","),
            Self::OpeningParen => write!(f, "("),
            Self::ClosingParen => write!(f, ")"),
            Self::Operator(op) => write!(f, "{op}"),
        }
    }
}
