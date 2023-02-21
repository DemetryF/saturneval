pub mod number_collector;
pub mod operator_collector;
pub mod special_collector;
pub mod word_collector;

use super::{CodeStream, TokenValue};

pub trait TokenCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue>;
}
