use std::collections::HashMap;

use crate::eval::Evaluator;

mod error;
pub mod eval;
mod lexer;
mod parser;

fn main() {
    let mut eval = Evaluator {
        functions: HashMap::new(),
        constants: HashMap::new(),
    };

    eval.functions.insert("call".into(), Box::new(|_| 1.0));

    println!("{:#?}", eval.eval("id()".into()));
}
