use std::io::stdin;
use simple_logger::SimpleLogger;

mod lexer;

fn main() {
    SimpleLogger::new().init().unwrap();

    loop {
        println!("> ");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let line = buffer.as_bytes();

        let l = lexer::Lexer::new(line);
        for tok in l {
            println!("{:?} ", tok);
        }
    }
}
