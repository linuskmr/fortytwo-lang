use std::env;
use std::io::{Read, stdin};
use std::sync::Arc;

use miette::NamedSource;

use fortytwo_lang::lexer::Lexer;
use fortytwo_lang::parser::Parser;

fn main() -> miette::Result<()> {
    let args: Vec<_> = env::args().collect();
    match args.get(1) {
        Some(_) => show_help(),
        _ => parser_from_stdin()?,
    }
    Ok(())
}

fn parser_from_stdin() -> miette::Result<()> {
    let mut sourcecode = String::new();
    stdin()
        .read_to_string(&mut sourcecode)
        .expect("Could not read sourcecode from stdin");
    let named_source = Arc::new(NamedSource::new("stdin", sourcecode.clone()));
    let lexer = Lexer::new(sourcecode.chars(), named_source.clone());
    let tokens = lexer.map(Result::unwrap);
    let parser = Parser::new(tokens, named_source.clone());
    for ast in parser {
        println!("{:#?}", ast?);
    }
    Ok(())
}

fn show_help() {
    println!(
        r#"FORTYTWO-LANG PARSER DUMP
Dumps the output of the parser.
Write your ftl sourcecode to stdin and end stdin by pressing CTRL+C.

USAGE:
    ftlpd"#
    )
}
