use parser::Parser;

mod error;
mod lexer;
mod parser;

fn main() {
    let mut parser = Parser::new("0 + 1 + 2 * 3 ^ call(4, 5, 6)".into()).unwrap();

    println!("{:#?}", parser.parse());
}
