use std::collections::HashMap;

use repl::Repl;

use crate::eval::Evaluator;

mod error;
pub mod eval;
mod lexer;
mod parser;
mod repl;

fn main() {
    let mut evaluator = Evaluator::default();

    Repl::new(evaluator).start();
}
