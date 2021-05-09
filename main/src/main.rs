use std::io::stdin;
use lexer;

fn main() {
    loop {
        println!("> ");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let line = buffer.as_bytes();

        let l = lexer::Lexer::new(line);
        let lines: Vec<_> = buffer.lines().collect();
        for tok in l {
            println!("Token {:?} from {:?} at line {}, columns {} til {}", tok.data, &lines[tok.position.line.clone()][tok.position.column.clone()], tok.position.line, tok.position.column.start(), tok.position.column.end());
        }
    }
}
