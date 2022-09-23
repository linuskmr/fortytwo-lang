use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use clap::Parser;
use fortytwolang::{lexer, Token};

use fortytwolang::lexer::Lexer;
use fortytwolang::source::{PositionContainer, Source};


/// FORTYTWO-LANG COMPILER
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
	#[clap(subcommand)]
	command: Command,
}

#[derive(Parser, Debug)]
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
	let lines = container.position.get_affected_lines();

	let mut s = String::new();

	for line in lines {
		s.push_str(&line);
		s.push('\n');
		s.push_str(&" ".repeat(container.position.position.start.column));
		let highlight_width = container.position.position.end.column - container.position.position.start.column + 1;
		s.push_str(&"^".repeat(highlight_width));
	}
	s
}

fn lex(path: PathBuf) -> Result<(), Box<dyn Error>> {
	let mut file = File::open(&path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;

	let source = Arc::new(Source::new(
		path.to_str().unwrap().to_string(),
		content,
	));

	let tokens = Lexer::new(source.iter());
	let tokens = tokens.collect::<Result<Vec<Token>, lexer::Error>>()?;
	for token in tokens {
		// Padding only works on string. Otherwise, the padding is applied to the struct, which doesn't handle it
		println!("{:<30} {:?}", token.position.to_string(), *token);
	}

	Ok(())
}

fn parse(_file: PathBuf) -> Result<(), Box<dyn Error>> {
	todo!("parse")
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
	let args = Args::parse();

	match args.command {
		Command::Lex { file: path } => lex(path),
		Command::Parse { file: path } => parse(path),
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
				},
				lexer::Error::IllegalSymbol(symbol) => {
					message = format!("{}\n{}", err, symbol.as_ref()
						.map(position_container_code)
						.unwrap_or("None".to_owned())
					);
				},
				lexer::Error::ParseNumberError(number_str) => {
					message = format!("{}\n{}", err, position_container_code(number_str));
				},
			}
		} else {
			message = err.to_string();
		}
		eprintln!("ERROR: {}", message);
		std::process::exit(1);
	}
}