use std::collections::HashMap;

use repl::Repl;

use crate::eval::Evaluator;

mod error;
pub mod eval;
mod lexer;
mod parser;
mod repl;

fn main() {
    let mut evaluator = Evaluator {
        functions: HashMap::new(),
        constants: HashMap::new(),
    };

    evaluator.functions.insert("call".into(), Box::new(|_| 1.0));

    Repl::new(evaluator).start();
}
