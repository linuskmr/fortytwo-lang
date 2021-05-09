mod lexer;
mod token;
mod position_container;
mod position_reader;
mod error;
mod ast;
mod parser;

use std::io::stdin;

struct StdinReader;

impl Iterator for StdinReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        println!("> ");
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        Some(line)
    }
}

fn main() {
    let stdin_reader = StdinReader{};
    let _lexer = lexer::Lexer::new(stdin_reader);
    let _parser = parser::Parser::new(_lexer);
    for tok in _parser {
        println!("Token {:?}", tok);
    }
}
