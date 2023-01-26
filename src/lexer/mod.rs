use crate::unexpected_char::UnexpectedChar;

use self::{
    code_stream::CodeStream,
    token::{Token, TokenValue},
    token_collector::{
        number_collector::NumberCollector, operator_collector::OperatorCollector,
        special_collector::SpecialCollector, word_collector::WordCollector, TokenCollector,
    },
};

mod code_stream;
pub mod operator;
pub mod token;
mod token_collector;

pub struct Lexer {
    collectors: Vec<Box<dyn TokenCollector>>,
    pub code_stream: CodeStream,
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

    pub fn next_token(&mut self) -> Result<Token, UnexpectedChar> {
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

    fn unexpected_char(&mut self) -> UnexpectedChar {
        UnexpectedChar::new(
            self.code_stream.accept().to_string(),
            self.code_stream.index - 1,
        )
    }

    fn skip_spaces(&mut self) {
        while !self.code_stream.is_eof() && self.code_stream.current().is_whitespace() {
            self.code_stream.accept();
        }
    }
}
