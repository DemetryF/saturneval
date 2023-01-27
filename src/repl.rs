use crate::{
    error::{Error, ErrorKind},
    eval::Evaluator,
};
use colored::Colorize;
use linefeed::{Interface, ReadResult};

pub struct Repl {
    evaluator: Evaluator,
    prefix: &'static str,
}

impl Repl {
    pub fn new(evaluator: Evaluator) -> Self {
        Self {
            evaluator,
            prefix: ">",
        }
    }

    pub fn start(&self) {
        Self::welcome();
        let reader = self.create_reader();

        while let ReadResult::Input(expr) = reader.read_line().unwrap() {
            let expr = expr.trim_end().to_string();
            if expr == "exit" {
                break;
            }
            self.eval_loop(expr);
        }
    }

    fn welcome() {
        println!(
            "Welcome to SaturnEval v{}.{}.{}.\nType \"exit\" for exit.",
            std::env::var("CARGO_PKG_VERSION_MAJOR").unwrap(),
            std::env::var("CARGO_PKG_VERSION_MINOR").unwrap(),
            std::env::var("CARGO_PKG_VERSION_PATCH").unwrap(),
        );
    }

    fn create_reader(&self) -> Interface<linefeed::DefaultTerminal> {
        let reader = Interface::new("SaturnEval").unwrap();

        reader
            .set_prompt((self.prefix.to_string() + " ").as_str())
            .unwrap();

        reader
    }

    fn eval_loop(&self, expr: String) {
        match self.evaluator.eval(expr) {
            Ok(number) => println!("{}", number),
            Err(error) => self.error(error),
        }
    }

    pub fn error(&self, error: Error) {
        let shift = " ".repeat(error.index + self.prefix.len() + 1);
        let mut arrow = "^".to_string();
        let prefix = "Error: ".red();
        let message = error.kind.to_string();

        match error.kind {
            ErrorKind::InvalidFunction(id) => arrow += tail(id.len() - 1).as_str(),
            ErrorKind::InvalidConst(id) => arrow += tail(id.len() - 1).as_str(),
            _ => {}
        }

        let arrow = arrow.magenta();

        println!("{shift}{arrow}\n{prefix}{message}");

        fn tail(len: usize) -> String {
            "~".repeat(len)
        }
    }
}
