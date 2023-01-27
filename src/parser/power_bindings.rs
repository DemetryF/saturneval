use crate::lexer::operator::Operator::{self, *};

pub struct PowerBindings;
impl PowerBindings {
    pub fn prefix(op: Operator) -> Result<usize, ()> {
        match op {
            Subtraction => Ok(7),
            _ => Err(()),
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
