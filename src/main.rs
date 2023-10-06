use anyhow::Context;
use fortytwolang::lexer::Lexer;
use fortytwolang::parser::Parser;
use fortytwolang::semantic_analyzer::SemanticAnalyzer;
use fortytwolang::source::{PositionContainer, Source, SourcePositionRange};
use fortytwolang::token::Token;
use fortytwolang::{emitter, lexer, parser, semantic_analyzer};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::{fs, io, process};

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

	SemCheck {
		/// The file to semantic check.
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

fn position_container_code(position: &SourcePositionRange) -> String {
	let affected_code = position.get_affected_lines();

	let mut s = String::new();

	for line in affected_code.lines() {
		s.push_str(&line);
		s.push('\n');
		s.push_str(&" ".repeat(position.position.start.column - 1));
		let highlight_width = position.position.end.column - position.position.start.column + 1;
		s.push_str(&"^".repeat(highlight_width));
	}
	s
}

fn lex(path: &Path) -> anyhow::Result<()> {
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

fn parse(path: &Path) -> anyhow::Result<()> {
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

fn format(path: &Path) -> anyhow::Result<()> {
	let content = fs::read_to_string(&path)?;

	let source = Arc::new(Source::new(path.to_str().unwrap().to_string(), content));
	let lexer = Lexer::new(source.iter());
	let tokens = lexer.collect::<Result<Vec<Token>, lexer::Error>>()?;

	let parser = Parser::new(tokens.into_iter());
	let ast_nodes = parser.collect::<Result<Vec<_>, _>>()?;

	emitter::Ftl::codegen(ast_nodes.into_iter(), Box::new(io::stdout()))?;
	Ok(())
}

fn sem_check(path: &Path) -> anyhow::Result<()> {
	let content = fs::read_to_string(&path)?;

	let source = Arc::new(Source::new(path.to_str().unwrap().to_string(), content));
	let lexer = Lexer::new(source.iter());
	let tokens = lexer.collect::<Result<Vec<Token>, lexer::Error>>()?;

	let parser = Parser::new(tokens.into_iter());
	let ast_nodes = parser.collect::<Result<Vec<_>, _>>()?;

	let sem_check: SemanticAnalyzer<semantic_analyzer::pass::GlobalSymbolScan> =
		SemanticAnalyzer::default();
	let sem_check: SemanticAnalyzer<semantic_analyzer::pass::TypeCheck> =
		sem_check.global_symbol_scan(ast_nodes.iter())?;
	sem_check.type_check(ast_nodes.iter())?;
	Ok(())
}

fn compile(path: &Path) -> anyhow::Result<()> {
	let content = fs::read_to_string(&path).context(format!("Reading ftl file `{:?}`", path))?;

	let source = Arc::new(Source::new(path.to_str().unwrap().to_string(), content));
	let lexer = Lexer::new(source.iter());
	let tokens = lexer
		.collect::<Result<Vec<Token>, lexer::Error>>()
		.context("Lexing error")?;

	let parser = Parser::new(tokens.into_iter());
	let ast_nodes = parser
		.collect::<Result<Vec<_>, _>>()
		.context("Parser error")?;

	// Compile to c code
	let c_code_output_path = Path::new(&path).with_extension("c");
	let c_code_output_file = File::create(&c_code_output_path).context(format!(
		"Creating output .c file `{:?}`",
		c_code_output_path
	))?;

	emitter::C::codegen(ast_nodes.into_iter(), Box::new(c_code_output_file))?;

	// Compile to executable
	let executable_output_path = Path::new(&path).with_extension("");
	let c_compile = process::Command::new("cc")
		.args([
			c_code_output_path.to_string_lossy().as_ref(),
			"-o",
			executable_output_path.to_string_lossy().as_ref(),
		])
		.output()
		.context("Invoking C compiler")?;
	if !c_compile.status.success() {
		io::stdout().write_all(&c_compile.stdout).unwrap();
		io::stderr().write_all(&c_compile.stderr).unwrap();
	}

	Ok(())
}

fn run(path: &Path) -> anyhow::Result<()> {
	compile(path)?;

	let executable = format!(
		"./{}",
		Path::new(&path).with_extension("").to_string_lossy()
	);
	let status_code = process::Command::new(&executable)
		.stdin(process::Stdio::piped())
		.stderr(process::Stdio::piped())
		.stdout(process::Stdio::piped())
		.spawn()
		.context(format!("Running executable `{}`", executable))?
		.wait()
		.context(format!("Waiting for executable `{}` to exit", executable))?;

	if !status_code.success() {
		eprintln!("Exited with status code {}", status_code);
	}

	Ok(())
}

fn main_() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();

	let args = <Args as clap::Parser>::parse();

	match args.command {
		Command::Lex { file: path } => lex(&path),
		Command::Parse { file: path } => parse(&path),
		Command::Fmt { file: path } => format(&path),
		Command::Compile { file: path } => compile(&path),
		Command::Run { file: path } => run(&path),
		Command::SemCheck { file: path } => sem_check(&path),
	}
}

fn main() {
	if let Err(err) = main_() {
		let mut message = String::new();

		if let Some(err) = err.downcast_ref::<lexer::Error>() {
			message += "LexerError\n";
			match err {
				lexer::Error::UnknownSymbol(symbol) => {
					message += &format!("{}\n{}", err, position_container_code(&symbol.position));
				}
				lexer::Error::IllegalSymbol(symbol) => {
					message += &format!(
						"{}\n{}",
						err,
						symbol
							.as_ref()
							.map(|s| position_container_code(&s.position))
							.unwrap_or_default()
					);
				}
				lexer::Error::ParseNumberError(number_str) => {
					message +=
						&format!("{}\n{}", err, position_container_code(&number_str.position));
				}
			}
		} else if let Some(err) = err.downcast_ref::<parser::Error>() {
			message += "ParserError\n";
			message += &format!("{}", err);
		} else if let Some(err) = err.downcast_ref::<semantic_analyzer::Error>() {
			message += "SemanticError\n";
			match err {
				semantic_analyzer::Error::Redeclaration {
					previous_declaration,
					new_declaration,
				} => {
					message += &format!(
						"{}\n{}",
						err,
						position_container_code(&new_declaration.name.position)
					)
				}
				semantic_analyzer::Error::UndeclaredVariable { name } => {
					message += &format!("{}\n{}", err, position_container_code(&name.position))
				}
				semantic_analyzer::Error::TypeMismatch { position, .. } => {
					message += &format!("{}\n{}", err, position_container_code(&position))
				}
				semantic_analyzer::Error::UndefinedFunctionCall { function } => {
					message += &format!(
						"{}\n{}",
						err,
						position_container_code(&function.name.position)
					)
				}
			}
		} else {
			message = err.to_string();
		}

		eprintln!("\nERROR\n{}", message);
		std::process::exit(1);
	}
}
