/*use ftllib::{lexer::Lexer, parser::Parser, runtime::Runtime};
use std::io::{stdin, stdout, Write};

struct StdinReader{
    line_nr: usize,
}

impl StdinReader {
    fn new() -> Self {
        Self {
            line_nr: 0,
        }
    }
}

impl Iterator for StdinReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.line_nr += 1;
        print!("In [{}]: ", self.line_nr);
        stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = stdin().read_line(&mut line).unwrap();
        match bytes_read {
            0 => None, // EOF
            _ => Some(line),
        }
    }
}

fn main() {
    let stdin_reader = StdinReader::new();
    let lexer = Lexer::new(stdin_reader);
    let parser = Parser::new(lexer);
    let mut runtime = Runtime::new();
    for parse_result in parser {
        // println!("Parse Result: {:#?}", parse_result);
        match parse_result {
            Err(err) => println!("{}", err),
            Ok(ast) => println!("Result: {}", runtime.execute_ast(ast)),
        }
    }
}
*/