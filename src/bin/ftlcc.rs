use std::fs::File;
use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::path::Path;
use std::{env, fs};
use std::sync::Arc;
use miette::miette;
use fortytwo_lang::ast::AstNode;
use fortytwo_lang::emitter_c::EmitterC;
use fortytwo_lang::parser::Parser;
use fortytwo_lang::lexer::Lexer;
use fortytwo_lang::position_reader::PositionReader;
use fortytwo_lang::token::Token;

fn main() {
    let args: Vec<_> = env::args().collect();
    let filepath = match args.get(1) {
        Some(filepath) => filepath,
        None => {
            show_help();
            return;
        },
    };

    let sourcecode = fs::read_to_string(filepath).expect("Could not read sourcecode from specified file");
    let target_path = Path::new(filepath).with_extension("c");
    let target_file = File::create(target_path).expect("Could not create target file");

    let named_source = Arc::new(miette::NamedSource::new(filepath.clone(), sourcecode.clone()));
    let position_reader = PositionReader::new(sourcecode.chars());
    let lexer = Lexer::new(position_reader, named_source.clone());
    // Result::unwrap as fn(ParseResult<Token>) -> Token: Convert fn item to fn pointer.
    // See https://users.rust-lang.org/t/puzzling-expected-fn-pointer-found-fn-item/46423/4
    let token_iter = lexer.map(Result::unwrap as fn(miette::Result<Token>) -> Token);
    let parser = Parser::new(token_iter, named_source.clone());
    let ast_iter = parser.map(Result::unwrap as fn(miette::Result<AstNode>) -> AstNode);
    EmitterC::codegen(ast_iter, BufWriter::new(target_file)).unwrap();
}


fn show_help() {
    println!(
        r#"FORTYTWO-LANG C COMPILER
Compiles ftl sourcecode to c sourcecode.

USAGE:
    ftlcc [FILE]

ARGUMENTS:
    - FILE: The path to the file you want to compile. ftlcc generates
    a file with the same name, but the extension `.c`.
    If FILE is not specified, the sourcecode is read from stdin until
    an EOF is read. The compiled c code is then printed to stdout.
"#
    )
}
