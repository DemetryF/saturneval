use derive_more::Constructor;

use crate::{
    error::{Error, ErrorKind},
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
        id: Id,
        args: Vec<Expr>,
    },
    Atom(Atom),
}

#[derive(Debug)]
pub enum Atom {
    Number(String),
    Id(Id),
}

#[derive(Debug, Constructor)]
pub struct Id {
    pub value: String,
    pub index: usize,
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
        let expr = self.expr_bp(0)?;
        self.token_stream.accept(&TokenValue::Eof)?;
        Ok(expr)
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
                self.call(Id::new(id, token.index))?
            }
            TokenValue::Operator(op) => self.unary(op, token.index)?,
            TokenValue::Number(num) => Expr::Atom(Atom::Number(num)),
            TokenValue::Id(id) => Expr::Atom(Atom::Id(Id::new(id, token.index))),
            _ => return Err(Self::fail(token)),
        })
    }

    fn fail(token: Token) -> Error {
        Error::new(ErrorKind::UnexpectedToken(token.value), token.index)
    }

    fn unary(&mut self, op: Operator, index: usize) -> Result<Expr, Error> {
        let r_bp = match PowerBindings::prefix(op) {
            Ok(r_bp) => r_bp,
            Err(()) => {
                return Err(Self::fail(Token {
                    value: TokenValue::Operator(op),
                    index,
                }))
            }
        };
        let rhs = Box::new(self.expr_bp(r_bp)?);

        Ok(Expr::Prefix { op, rhs })
    }

    fn call(&mut self, id: Id) -> Result<Expr, Error> {
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

        args.push(self.expr_bp(0)?);

        while self.token_stream.check(&TokenValue::Comma) {
            self.token_stream.skip()?;
            args.push(self.parse()?);
        }

        Ok(args)
    }
}
