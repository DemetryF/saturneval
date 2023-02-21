use crate::{
    error::Error,
    lexer::Operator::*,
    parser::{Atom, Call, Expr, Infix, Prefix},
};

use super::Evaluator;

pub trait Eval {
    fn eval(self, evaluator: &Evaluator) -> Result<f64, Error>;
}

impl Eval for Call {
    fn eval(self, evaluator: &Evaluator) -> Result<f64, Error> {
        match evaluator.env.get_function(self.id) {
            Ok(f) => Ok((f.call)(
                self.args
                    .into_iter()
                    .map(|x| x.eval(evaluator))
                    .collect::<Result<Vec<f64>, Error>>()?,
            )),
            Err(error) => Err(error),
        }
    }
}

impl Eval for Expr {
    fn eval(self, evaluator: &Evaluator) -> Result<f64, Error> {
        match self {
            Expr::Prefix(prefix) => prefix.eval(evaluator),
            Expr::Infix(infix) => infix.eval(evaluator),
            Expr::Call(call) => call.eval(evaluator),
            Expr::Atom(Atom::Id(id)) => evaluator.env.get_const(id),
            Expr::Atom(Atom::Number(num)) => Ok(num.parse().unwrap()),
        }
    }
}

impl Eval for Infix {
    fn eval(self, evaluator: &Evaluator) -> Result<f64, Error> {
        let left = self.lhs.eval(evaluator)?;
        let right = self.rhs.eval(evaluator)?;

        fn whole_division(left: f64, right: f64) -> f64 {
            ((left as i64) / (right as i64)) as f64
        }

        Ok(match self.op {
            Addition => left + right,
            Subtraction => left - right,
            Multiplication => left * right,
            Division => left / right,
            WholeDivision => whole_division(left, right),
            ModuloDivision => left % right,
            Exponentiation => left.powf(right),
        })
    }
}

impl Eval for Prefix {
    fn eval(self, evaluator: &Evaluator) -> Result<f64, Error> {
        let value = self.rhs.eval(evaluator)?;
        Ok(-value)
    }
}
