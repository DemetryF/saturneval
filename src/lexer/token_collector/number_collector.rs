use crate::lexer::{CodeStream, TokenCollector, TokenValue};

pub struct NumberCollector;
impl NumberCollector {
    fn is_digit(code: &CodeStream) -> bool {
        code.current().is_ascii_digit()
    }

    fn lex_number_literal(code: &mut CodeStream) -> String {
        let mut str = String::new();

        while !code.is_eof() && Self::is_digit(code) {
            str += code.accept().to_string().as_mut()
        }

        str
    }
}

impl TokenCollector for NumberCollector {
    fn try_collect(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_digit(code) && !code.check('.') {
            return None;
        }

        let mut source = Self::lex_number_literal(code);

        if code.check('.') {
            source += code.accept().to_string().as_mut();
            source += Self::lex_number_literal(code).as_mut();
        }

        Some(TokenValue::Number(source))
    }
}
