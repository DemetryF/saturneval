use crate::{error::Error, parser::Parser};

use self::{env::Env, eval::Eval};

pub mod env;
mod eval;

pub struct Evaluator {
    pub env: Env,
}

impl Evaluator {
    pub fn eval(&self, code: String) -> Result<f64, Error> {
        Parser::new(code)?.parse()?.eval(self)
    }
}
