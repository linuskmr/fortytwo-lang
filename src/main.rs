use fortytwolang::lexer::Lexer;
use fortytwolang::parser::Parser;
use fortytwolang::source::{PositionContainer, Source};
use fortytwolang::token::Token;
use fortytwolang::{emitter, lexer};
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs, io};

/// FORTYTWO-LANG COMPILER
#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Parser, Debug)]
enum Command {
    /// Dumps the output of the lexer.
    Lex {
        /// The file to lex.
        #[clap(parse(from_os_str))]
        file: std::path::PathBuf,
    },

    /// Dumps the output of the parser.
    Parse {
        /// The file to parse.
        #[clap(parse(from_os_str))]
        file: std::path::PathBuf,
    },

    /// Formats the code.
    Fmt {
        /// The file to format.
        #[clap(parse(from_os_str))]
        file: std::path::PathBuf,
    },

    /// Compiles to C sourcecode.
    IntermediateCompileToC {
        /// The file to compile.
        #[clap(parse(from_os_str))]
        file: std::path::PathBuf,
    },

    /// Compiles to an executable.
    Compile {
        /// The file to compile.
        #[clap(parse(from_os_str))]
        file: std::path::PathBuf,
    },

    /// Compile and execute.
    Run {
        /// The file to compile.
        #[clap(parse(from_os_str))]
        file: std::path::PathBuf,
    },
}

fn position_container_code<T>(container: &PositionContainer<T>) -> String {
    let affected_code = container.position.get_affected_code();

    let mut s = String::new();

    for line in affected_code.lines() {
        s.push_str(&line);
        s.push('\n');
        s.push_str(&" ".repeat(container.position.position.start.column));
        let highlight_width =
            container.position.position.end.column - container.position.position.start.column + 1;
        s.push_str(&"^".repeat(highlight_width));
    }
    s
}

fn lex(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&path)?;

    let source = Arc::new(Source::new(path.to_str().unwrap().to_string(), content));

    let tokens = Lexer::new(source.iter());
    let tokens = tokens.collect::<Result<Vec<Token>, lexer::Error>>()?;
    for token in tokens {
        // Padding only works on string. Otherwise, the padding is applied to the struct, which doesn't handle it
        println!("{:<30} {:?}", token.position.to_string(), *token);
    }

    Ok(())
}

fn parse(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&path)?;

    let source = Arc::new(Source::new(path.to_str().unwrap().to_string(), content));
    let lexer = Lexer::new(source.iter());
    let tokens = lexer.collect::<Result<Vec<Token>, lexer::Error>>()?;

    let ast_nodes = Parser::new(tokens.into_iter());
    for ast_node in ast_nodes {
        // Padding only works on string. Otherwise, the padding is applied to the struct, which doesn't handle it
        match ast_node {
            Ok(ast_node) => println!("{:#?}", ast_node),
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }

    Ok(())
}

fn format(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&path)?;

    let source = Arc::new(Source::new(path.to_str().unwrap().to_string(), content));
    let lexer = Lexer::new(source.iter());
    let tokens = lexer.collect::<Result<Vec<Token>, lexer::Error>>()?;

    let parser = Parser::new(tokens.into_iter());
    let ast_nodes = parser.collect::<Result<Vec<_>, _>>()?;

    emitter::ftl::FtlEmitter::codegen(ast_nodes.into_iter(), io::stdout())?;
    Ok(())
}

fn intermediate_compile_to_c(_file: PathBuf) -> Result<(), Box<dyn Error>> {
    todo!("intermediate_compile_to_c")
}

fn compile(_file: PathBuf) -> Result<(), Box<dyn Error>> {
    todo!("compile")
}

fn run(_file: PathBuf) -> Result<(), Box<dyn Error>> {
    todo!("run")
}

fn main_() -> Result<(), Box<dyn Error>> {
    let args = <Args as clap::Parser>::parse();

    match args.command {
        Command::Lex { file: path } => lex(path),
        Command::Parse { file: path } => parse(path),
        Command::Fmt { file: path } => format(path),
        Command::IntermediateCompileToC { file: path } => intermediate_compile_to_c(path),
        Command::Compile { file: path } => compile(path),
        Command::Run { file: path } => run(path),
    }
}

fn main() {
    if let Err(err) = main_() {
        let message;

        if let Some(err) = err.downcast_ref::<lexer::Error>() {
            match err {
                lexer::Error::UnknownSymbol(symbol) => {
                    message = format!("{}\n{}", err, position_container_code(symbol));
                }
                lexer::Error::IllegalSymbol(symbol) => {
                    message = format!(
                        "{}\n{}",
                        err,
                        symbol
                            .as_ref()
                            .map(position_container_code)
                            .unwrap_or("None".to_owned())
                    );
                }
                lexer::Error::ParseNumberError(number_str) => {
                    message = format!("{}\n{}", err, position_container_code(number_str));
                }
            }
        } else {
            message = err.to_string();
        }
        eprintln!("ERROR: {}", message);
        std::process::exit(1);
    }
}
