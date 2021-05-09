mod lexer;
mod token;
mod position_container;
mod position_reader;
mod error;
mod ast;

use std::io::stdin;
use std::marker;

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
    for tok in _lexer {
        println!("Token {:?}", tok);
    }
}
