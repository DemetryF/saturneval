use crate::{
    error::{Error, ErrorKind},
    lexer::{Lexer, Token, TokenValue},
};

pub struct TokenStream {
    pub lexer: Lexer,

    current: Token,
    following: Option<Token>,
}

impl TokenStream {
    pub fn new(code: String) -> Result<Self, Error> {
        let mut lexer = Lexer::new(code);

        let current = lexer.next()?;

        Ok(Self {
            lexer,
            current,
            following: None,
        })
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn skip(&mut self) -> Result<Token, Error> {
        let token = self.current.clone();

        match self.following.take() {
            Some(token) => self.current = token,
            None => self.current = self.lexer.next()?,
        }

        Ok(token)
    }

    pub fn check(&self, value: &TokenValue) -> bool {
        &self.current().value == value
    }

    pub fn accept(&mut self, value: &TokenValue) -> Result<(), Error> {
        if self.check(value) {
            self.skip()?;
            return Ok(());
        }

        Err(self.fail())
    }

    pub fn fail(&self) -> Error {
        Error::new(
            ErrorKind::UnexpectedToken(self.current().value.clone()),
            self.current.index,
        )
    }
}
