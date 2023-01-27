use std::fmt::Display;

use derive_more::Constructor;

use crate::lexer::token::TokenValue;

#[derive(Debug, Constructor)]
pub struct Error {
    pub kind: ErrorKind,
    pub index: usize,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedChar(char),
    UnexpectedToken(TokenValue),
    InvalidFunction(String),
    InvalidConst(String),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::UnexpectedChar(char) => write!(f, "unexpected char '{char}'"),
            ErrorKind::UnexpectedToken(value) => write!(f, "unexpected token \"{value}\""),
            ErrorKind::InvalidConst(id) => write!(f, "cannot find \"{id}\""),
            ErrorKind::InvalidFunction(id) => write!(f, "function \"{id}\" is not defined"),
        }
    }
}
