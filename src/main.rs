use evaluator::{env::Env, Evaluator};
use repl::Repl;

mod error;
pub mod evaluator;
mod lexer;
mod parser;
mod repl;

fn main() {
    let evaluator = Evaluator {
        env: Env::default(),
    };

    Repl::new(evaluator).start();
}
