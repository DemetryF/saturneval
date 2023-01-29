use env::Env;
use repl::Repl;

use crate::eval::Evaluator;

mod env;
mod error;
pub mod eval;
mod lexer;
mod parser;
mod repl;

fn main() {
    let evaluator = Evaluator {
        env: Env::default(),
    };

    Repl::new(evaluator).start();
}
