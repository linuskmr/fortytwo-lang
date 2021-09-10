use ftllib::{lexer::Lexer, parser::Parser, runtime::Runtime};
use std::io::{stdin, stdout, Write, BufReader, Read};
use std::str::Chars;
use std::fs::File;
use std::io;


fn main() -> io::Result<()> {
    let source_file = File::open("sourcecode.ftl")?;
    let reader = BufReader::new(source_file);
    let lexer = Lexer::new(reader);
    let parser = Parser::new(lexer);
    let mut runtime = Runtime::new();
    for parse_result in parser {
        // println!("Parse Result: {:#?}", parse_result);
        match parse_result {
            Err(err) => println!("{}", err),
            Ok(ast) => println!("Result: {}", runtime.execute_ast(ast)),
        }
    }
    Ok(())
}
