use std::env;
use std::io::{Read, stdin};
use miette::NamedSource;
use fortytwo_lang::lexer::Lexer;
use fortytwo_lang::position_reader::PositionReader;

fn main() -> miette::Result<()> {
    let args: Vec<_> = env::args().collect();
    match args.get(1) {
        Some(arg) if arg == "--help" || arg == "-h" => show_help(),
        _ => lexer_from_stdin()?,
    }
    Ok(())
}

fn lexer_from_stdin() -> miette::Result<()> {
    let mut sourcecode = String::new();
    stdin().read_to_string(&mut sourcecode).expect("Could not read sourcecode from stdin");
    let named_source = NamedSource::new("stdin", sourcecode.clone());
    let lexer = Lexer::new(PositionReader::new(sourcecode.chars()), named_source);
    for symbol in lexer {
        if let Err(e) = symbol {
            return Err(e.into())
        }
        println!("{:?}", symbol)
    }
    Ok(())
}

fn show_help() {
    println!(
        r#"FORTYTWO-LANG LEXER DUMP
Dumps the output of the lexer.
Write ftl sourcecode to stdin and end stdin by pressing CTRL+C.

USAGE:
    ftlld
"#
    )
}