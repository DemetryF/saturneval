use strum::IntoEnumIterator;

use crate::lexer::{
    code_stream::CodeStream, operator::Operator, token::TokenValue, token_collector::TokenCollector,
};

pub struct OperatorCollector;

impl TokenCollector for OperatorCollector {
    fn try_next(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        for op in Operator::iter() {
            let value = char::from(op);

            if code_stream.check(value) {
                code_stream.accept();
                return Some(TokenValue::Operator(op));
            }
        }

        None
    }
}
