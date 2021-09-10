//! Runs the lexer on the provided ftl sourcecode file and prints its result. Provided the path to the file as first
//! command line argument.

use fortytwo_lang::lexer::Lexer;
use fortytwo_lang::position_reader::PositionReader;
use std::{env, fs};

/// Runs the lexer on the provided ftl sourcecode file and prints its result. Provided the path to the file as first
/// command line argument.
fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let path = args.get(1).expect("Missing required cli argument: Path to ftl source file");
    let file_contents = fs::read_to_string(path).unwrap();
    let position_reader = PositionReader::new(file_contents.chars());
    let lexer = Lexer::new(position_reader);
    for symbol in lexer {
        // println!("{}", symbol.map_or(String::from("None"), |symbol| symbol.to_string()));
        println!("{:?}", symbol)
    }
}
