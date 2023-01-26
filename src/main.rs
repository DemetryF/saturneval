use lexer::{token::TokenValue, Lexer};

mod lexer;
mod unexpected_char;

fn main() {
    let mut a = Lexer::new("2 + 2 * 2 ^ call(1, 2, 3)".into());

    loop {
        let token = a.next_token();

        match token {
            Ok(token) if token.value == TokenValue::Eof => break,
            Ok(token) => println!("{:#?}", token),
            Err(error) => println!("{:#?}", error),
        }
    }
}
