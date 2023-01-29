use crate::{
    env::Env,
    error::Error,
    lexer::operator::Operator::*,
    parser::{Atom, Expr, Parser},
};

pub struct Evaluator {
    pub env: Env,
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
            Expr::Call { id, args } => match self.env.get_function(id) {
                Ok(f) => Ok((f.call)(
                    args.into_iter()
                        .map(|x| self.eval_expr(x))
                        .collect::<Result<Vec<f64>, Error>>()?,
                )),
                Err(_) => todo!(),
            },
            Expr::Atom(Atom::Id(id)) => self.env.get_const(id),
            Expr::Atom(Atom::Number(num)) => Ok(num.parse().unwrap()),
        }
    }
}
