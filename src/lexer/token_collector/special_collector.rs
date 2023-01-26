use crate::lexer::{code_stream::CodeStream, token::TokenValue, token_collector::TokenCollector};

pub struct SpecialCollector;

impl TokenCollector for SpecialCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        let value = Some(match code.current() {
            ',' => TokenValue::Comma,
            '(' => TokenValue::OpeningParen,
            ')' => TokenValue::ClosingParen,

            _ => return None,
        });

        code.accept();

        value
    }
}
