use evaluator::{env::Env, Evaluator};
use repl::Repl;

mod error;
mod evaluator;
mod lexer;
mod parser;
mod repl;

fn main() {
    let evaluator = Evaluator {
        env: Env::new_with_std(),
    };

    Repl::new(evaluator).start();
}
