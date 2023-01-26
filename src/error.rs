use crate::lexer::token::Token;

#[derive(Debug)]
pub enum Error {
    UnexpectedChar { value: char, index: usize },
    UnexpectedToken(Token),
}
