use crate::{
    error::Error,
    lexer::operator::Operator::{self, *},
};

pub struct PowerBindings;
impl PowerBindings {
    pub fn prefix(op: Operator) -> Result<usize, Error> {
        match op {
            Subtraction => Ok(7),

            _ => Err(Error::UnexpectedChar {
                value: char::from(op.to_owned()),
                index: 0,
            }),
        }
    }

    pub fn infix(op: Operator) -> (usize, usize) {
        match op {
            Division | Multiplication | WholeDivision | ModuloDivision => (3, 4),
            Subtraction | Addition => (1, 2),
            Exponentiation => (6, 5),
        }
    }
}
