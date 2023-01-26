use crate::{
    error::Error,
    lexer::{
        operator::Operator,
        token::{Token, TokenValue},
    },
};

use self::{power_bindings::PowerBindings, token_stream::TokenStream};

mod power_bindings;
mod token_stream;

#[derive(Debug)]
pub enum Expr {
    Prefix {
        op: Operator,
        rhs: Box<Expr>,
    },
    Infix {
        lhs: Box<Expr>,
        op: Operator,
        rhs: Box<Expr>,
    },
    Call {
        id: String,
        args: Vec<Expr>,
    },
    Atom(Atom),
}

#[derive(Debug)]
pub enum Atom {
    Number(String),
    Id(String),
}

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn new(code: String) -> Result<Self, Error> {
        Ok(Self {
            token_stream: TokenStream::new(code)?,
        })
    }

    pub fn parse(&mut self) -> Result<Expr, Error> {
        self.expr_bp(0)
    }

    fn expr_bp(&mut self, min_bp: usize) -> Result<Expr, Error> {
        let mut lhs = self.fact()?;

        while let TokenValue::Operator(op) = self.token_stream.current().value {
            let (l_bp, r_bp) = PowerBindings::infix(op);

            if l_bp < min_bp {
                break;
            }

            self.token_stream.skip()?;

            let rhs = self.expr_bp(r_bp)?;

            lhs = Expr::Infix {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        }

        Ok(lhs)
    }

    fn fact(&mut self) -> Result<Expr, Error> {
        let token = self.token_stream.skip()?;

        Ok(match token.value.clone() {
            TokenValue::Id(id) if self.token_stream.check(&TokenValue::OpeningParen) => {
                self.call(id)?
            }
            TokenValue::Operator(op) => self.unary(op)?,
            TokenValue::Number(num) => Expr::Atom(Atom::Number(num)),
            TokenValue::Id(id) => Expr::Atom(Atom::Id(id)),
            _ => return Err(Self::fail(token)),
        })
    }

    fn fail(token: Token) -> Error {
        Error::UnexpectedToken(token)
    }

    fn unary(&mut self, op: Operator) -> Result<Expr, Error> {
        let r_bp = PowerBindings::prefix(op)?;
        let rhs = Box::new(self.expr_bp(r_bp)?);

        Ok(Expr::Prefix { op, rhs })
    }

    fn call(&mut self, id: String) -> Result<Expr, Error> {
        self.token_stream.accept(&TokenValue::OpeningParen)?;
        let args = self.call_args()?;
        self.token_stream.accept(&TokenValue::ClosingParen)?;

        Ok(Expr::Call { id, args })
    }

    fn call_args(&mut self) -> Result<Vec<Expr>, Error> {
        let mut args = Vec::new();

        if self.token_stream.check(&TokenValue::ClosingParen) {
            return Ok(args);
        }

        args.push(self.parse()?);

        while self.token_stream.check(&TokenValue::Comma) {
            self.token_stream.skip()?;
            args.push(self.parse()?);
        }

        Ok(args)
    }
}
