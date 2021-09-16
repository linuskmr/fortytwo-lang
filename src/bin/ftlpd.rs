use std::io::{stdin, Read};
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs, mem};

use fortytwo_lang::parser::sourcecode_to_parser;

fn from_filepath(filepath: &String) -> String {
    println!("Parsedump from {}:", filepath);
    fs::read_to_string(filepath).unwrap()
}

fn from_stdin() -> String {
    println!("Write your sourcecode here and press CTRL+D to send an EOF");
    let mut sourcecode = String::new();
    stdin().read_to_string(&mut sourcecode).unwrap();
    println!("\nParsedump from stdin:");
    sourcecode
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let sourcecode = match args.get(1) {
        Some(filepath) => from_filepath(filepath),
        None => from_stdin(),
    };
    let parser = sourcecode_to_parser(sourcecode.chars());
    for x in parser {
        println!("{:#?}", x);
    }
}
