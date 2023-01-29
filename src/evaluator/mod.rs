pub mod env;
mod eval;

use crate::{error::Error, evaluator::eval::Eval, parser::Parser, Env};

pub struct Evaluator {
    pub env: Env,
}

impl Evaluator {
    pub fn eval(&self, code: String) -> Result<f64, Error> {
        Parser::new(code)?.parse()?.eval(self)
    }
}
