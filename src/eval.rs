use std::collections::HashMap;

use crate::{
    error::Error,
    lexer::operator::Operator::*,
    parser::{Atom, Expr, Parser},
};

type Function = dyn Fn(Vec<f64>) -> f64;

pub struct Evaluator {
    pub functions: HashMap<String, Box<Function>>,
    pub constants: HashMap<String, f64>,
}

impl Evaluator {
    pub fn eval(&self, code: String) -> Result<f64, Error> {
        let tree = Parser::new(code)?.parse()?;
        self.eval_expr(tree)
    }

    fn eval_expr(&self, expr: Expr) -> Result<f64, Error> {
        match expr {
            Expr::Prefix { op: _, rhs } => {
                let value = self.eval_expr(*rhs)?;
                Ok(-value)
            }
            Expr::Infix { lhs, op, rhs } => {
                let left = self.eval_expr(*lhs)?;
                let right = self.eval_expr(*rhs)?;

                fn whole_division(left: f64, right: f64) -> f64 {
                    ((left as i64) / (right as i64)) as f64
                }

                Ok(match op {
                    Addition => left + right,
                    Subtraction => left - right,
                    Multiplication => left * right,
                    Division => left / right,
                    WholeDivision => whole_division(left, right),
                    ModuloDivision => left % right,
                    Exponentiation => left.powf(right),
                })
            }
            Expr::Call { id, args } => match self.functions.get(&id) {
                None => Err(Error::InvalidFunction(id)),
                Some(f) => {
                    let mut call_args = Vec::new();

                    for arg in args {
                        call_args.push(self.eval_expr(arg)?);
                    }

                    Ok(f(call_args))
                }
            },
            Expr::Atom(Atom::Id(id)) => self.get_const(id),
            Expr::Atom(Atom::Number(num)) => Ok(num.parse().unwrap()),
        }
    }

    fn get_const(&self, id: String) -> Result<f64, Error> {
        match self.constants.get(&id) {
            Some(value) => Ok(*value),
            None => Err(Error::InvalidConst(id)),
        }
    }
}
