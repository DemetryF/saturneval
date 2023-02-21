use crate::lexer::{CodeStream, TokenCollector, TokenValue};

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
