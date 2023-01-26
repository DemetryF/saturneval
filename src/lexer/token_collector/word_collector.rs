use crate::lexer::{code_stream::CodeStream, token::TokenValue, token_collector::TokenCollector};

pub struct WordCollector;

impl WordCollector {
    fn is_word_char(code: &CodeStream) -> bool {
        code.current().is_ascii_alphanumeric()
    }

    fn lex_word(code: &mut CodeStream) -> String {
        let mut str = code.accept().to_string();

        while !code.is_eof() && Self::is_word_char(code) {
            str += code.accept().to_string().as_mut();
        }

        str
    }
}

impl TokenCollector for WordCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        if !code.current().is_alphabetic() {
            return None;
        }

        let source = Self::lex_word(code);

        Some(TokenValue::Id(source))
    }
}
