use std::collections::HashMap;

use crate::{
    error::{Error, ErrorKind},
    parser::expr::Id,
};

pub struct Function {
    pub args_count: usize,
    pub call: Box<dyn Fn(Vec<f64>) -> f64>,
}

pub struct Env {
    pub functions: HashMap<String, Function>,
    pub constants: HashMap<String, f64>,
}

impl Env {
    pub fn empty() -> Self {
        Self {
            functions: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn new_with_std() -> Self {
        let mut default = Self::empty();

        default.add_constant("pi", std::f64::consts::PI);
        default.add_constant("e", std::f64::consts::E);

        // trigonometric
        default.add_function("cos", 1, |x| x[0].cos());
        default.add_function("sin", 1, |x| x[0].sin());
        default.add_function("tan", 1, |x| x[0].tan());

        // inverse trigonometric
        default.add_function("acos", 1, |x| x[0].acos());
        default.add_function("asin", 1, |x| x[0].asin());
        default.add_function("atan", 1, |x| x[0].atan());

        // log
        default.add_function("log", 2, |x| x[1].log(x[0]));
        default.add_function("ln", 1, |x| x[0].ln());
        default.add_function("ld", 1, |x| x[0].log2());
        default.add_function("lg", 1, |x| x[0].log10());
        default.add_function("exp", 1, |x| x[0].exp());

        // rounding
        default.add_function("round", 1, |x| x[0].round());
        default.add_function("ceil", 1, |x| x[0].ceil());
        default.add_function("floor", 1, |x| x[0].floor());
        default.add_function("trunc", 1, |x| x[0].trunc());

        // other
        default.add_function("abs", 1, |x| x[0].abs());
        default.add_function("sign", 1, |x| x[0].signum());
        default.add_function("sqrt", 1, |x| x[0].sqrt());
        default.add_function("cbrt", 1, |x| x[0].cbrt());
        default.add_function("hypot", 2, |x| x[0].hypot(x[1]));

        default
    }

    pub fn add_function<F>(&mut self, name: &str, args_count: usize, f: F)
    where
        F: Fn(Vec<f64>) -> f64 + 'static,
    {
        self.functions.insert(
            name.into(),
            Function {
                args_count,
                call: Box::new(f),
            },
        );
    }

    pub fn add_constant(&mut self, name: &str, value: f64) {
        self.constants.insert(name.into(), value);
    }

    pub fn get_const(&self, id: Id) -> Result<f64, Error> {
        match self.constants.get(&id.value) {
            Some(value) => Ok(*value),
            None => Err(Error::new(ErrorKind::InvalidConst(id.value), id.index)),
        }
    }

    pub fn get_function(&self, id: Id) -> Result<&Function, Error> {
        match self.functions.get(&id.value) {
            None => Err(Error::new(ErrorKind::InvalidFunction(id.value), id.index)),
            Some(f) => Ok(f),
        }
    }
}
