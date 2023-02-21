use crate::error::{Error, ErrorKind};

use self::{
    code_stream::CodeStream,
    token_collector::{
        number_collector::NumberCollector, operator_collector::OperatorCollector,
        special_collector::SpecialCollector, word_collector::WordCollector, TokenCollector,
    },
};

pub use self::{
    operator::Operator,
    token::{Token, TokenValue},
};

mod code_stream;
pub mod operator;
pub mod token;
mod token_collector;

pub struct Lexer {
    pub code_stream: CodeStream,
    collectors: Vec<Box<dyn TokenCollector>>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            code_stream: CodeStream::new(code),
            collectors: vec![
                Box::new(NumberCollector),
                Box::new(OperatorCollector),
                Box::new(SpecialCollector),
                Box::new(WordCollector),
            ],
        }
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        self.skip_spaces();

        let index = self.code_stream.index;

        if self.code_stream.is_eof() {
            return Ok(Token::new(TokenValue::Eof, index));
        }

        for collector in self.collectors.iter_mut() {
            if let Some(token_value) = collector.try_next(&mut self.code_stream) {
                return Ok(Token::new(token_value, index));
            }
        }

        Err(self.unexpected_char())
    }

    pub fn unexpected_char(&mut self) -> Error {
        Error::new(
            ErrorKind::UnexpectedChar(self.code_stream.current()),
            self.code_stream.index,
        )
    }

    fn skip_spaces(&mut self) {
        while !self.code_stream.is_eof() && self.code_stream.current().is_whitespace() {
            self.code_stream.accept();
        }
    }
}
