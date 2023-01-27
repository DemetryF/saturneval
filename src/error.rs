use crate::{lexer::token::Token, parser::Id};

#[derive(Debug)]
pub enum Error {
    UnexpectedChar { value: char, index: usize },
    UnexpectedToken(Token),
    InvalidFunction(Id),
    InvalidConst(Id),
}
