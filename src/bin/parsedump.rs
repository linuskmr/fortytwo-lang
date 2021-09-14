use std::{env, fs};
use fortytwo_lang::position_reader::PositionReader;
use fortytwo_lang::lexer::Lexer;
use fortytwo_lang::parser::sourcecode_to_parser;
use std::io::{stdin, Read};


fn from_filepath(filepath: &String) {
    println!("Parsedump from {}:", filepath);
    let sourcecode = fs::read_to_string(filepath).unwrap();
    let parser = sourcecode_to_parser(sourcecode.chars());
    for x in parser {
        println!("{:#?}", x);
    }
    // println!("{:#?}", parser.collect::<Vec<_>>());
}

fn from_stdin() {
    println!("Write your sourcecode here and press CTRL+D to send an EOF");
    let mut sourcecode = String::new();
    stdin().read_to_string(&mut sourcecode).unwrap();
    let parser = sourcecode_to_parser(sourcecode.chars());
    println!("\nParsedump from stdin:");
    for x in parser.take(10) {
        println!("{:#?}", x);
    }
    // println!("{:#?}", parser.collect::<Vec<_>>())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.get(1) {
        Some(filepath) => from_filepath(filepath),
        None => from_stdin(),
    }
}