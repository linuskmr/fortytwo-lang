//! Command line interface to the fortytwo-lang compiler.

use std::{fs::File, io, io::Write, os::unix::process::CommandExt, path::Path, process};

use anyhow::Context;
use fortytwolang::{
	emitter,
	lexer::{self},
	parser::{self, Error},
	semantic_analyzer::{self},
	source::SourcePositionRange,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;

fn main() {
	tracing_subscriber::Registry::default()
		.with(tracing_subscriber::EnvFilter::from_default_env())
		/*.with(
			tracing_subscriber::fmt::layer()
				.with_file(true)
				.with_line_number(true),
		)*/
		.with(tracing_tree::HierarchicalLayer::new(2).with_targets(true).with_bracketed_fields(true))
		.init();

	let args = <cli::Args as clap::Parser>::parse();

	let result = match args.command {
		cli::Command::Compile { file: path } => compile(&path),
		cli::Command::Run { file: path } => run(&path),
		cli::Command::Fmt { file: path } => format(&path),
	};

	if let Err(err) = result {
		print_error(err);
		// TODO: Use [`process::ExitCode::Failure.exit_process()`](https://doc.rust-lang.org/beta/std/process/struct.ExitCode.html#method.exit_process) when stable
		process::exit(1);
	}
}

/// Formats FTL source code using the FTL emitter.
fn format(path: &Path) -> anyhow::Result<()> {
	let ast_nodes = fortytwolang::compiler_pipeline(path)?;

	emitter::Ftl::codegen(ast_nodes.into_iter(), Box::new(io::stdout()))?;
	Ok(())
}

/// Compiles FTL source code to a C executable.
fn compile(path: &Path) -> anyhow::Result<()> {
	let ast_nodes = fortytwolang::compiler_pipeline(path)?;

	// Compile to c code
	let c_code_output_path = Path::new(&path).with_extension("c");
	let c_code_output_file =
		File::create(&c_code_output_path).context(format!("Creating output .c file `{:?}`", c_code_output_path))?;

	emitter::C::codegen(ast_nodes.into_iter(), Box::new(c_code_output_file))?;

	// Compile to executable
	let executable_output_path = Path::new(&path).with_extension("");
	let c_compile = process::Command::new("cc")
		.args([c_code_output_path.to_string_lossy().as_ref(), "-o", executable_output_path.to_string_lossy().as_ref()])
		.output()
		.context("Invoking C compiler")?;
	if !c_compile.status.success() {
		io::stdout().write_all(&c_compile.stdout).unwrap();
		io::stderr().write_all(&c_compile.stderr).unwrap();
	}

	Ok(())
}

/// Compiles and runs the executable.
fn run(path: &Path) -> anyhow::Result<()> {
	compile(path)?;

	let executable = format!("./{}", Path::new(&path).with_extension("").to_string_lossy());
	let executing_err = process::Command::new(&executable)
		.stdin(process::Stdio::piped())
		.stderr(process::Stdio::piped())
		.stdout(process::Stdio::piped())
		.exec();
	Result::Err(executing_err) // anyhow.context expects a Result
		.context("Running executable")
}

fn print_error(err: anyhow::Error) {
	let mut message = String::new();

	if let Some(err) = err.downcast_ref::<lexer::Error>() {
		message += "LexerError\n";
		match err {
			lexer::Error::UnknownSymbol(symbol) => {
				message += &format!("{}\n{}", err, highlight_position_range(&symbol.position));
			},
			lexer::Error::IllegalSymbol(symbol) => {
				message += &format!(
					"{}\n{}",
					err,
					symbol.as_ref().map(|s| highlight_position_range(&s.position)).unwrap_or_default()
				);
			},
			lexer::Error::ParseNumberError(number_str) => {
				message += &format!("{}\n{}", err, highlight_position_range(&number_str.position));
			},
		}
	} else if let Some(err) = err.downcast_ref::<parser::Error>() {
		message += "ParserError\n";
		match err {
			Error::ExpectedToken { found, .. } => {
				message += &format!(
					"{}\n{}",
					err,
					found.as_ref().map(|found| { highlight_position_range(&found.position) }).unwrap_or_default()
				);
			},
			Error::IllegalToken { token, .. } => {
				message += &format!(
					"{}\n{}",
					err,
					token.as_ref().map(|found| { highlight_position_range(&found.position) }).unwrap_or_default()
				);
			},
		}
	} else if let Some(err) = err.downcast_ref::<semantic_analyzer::Error>() {
		message += "SemanticError\n";
		match err {
			semantic_analyzer::Error::Redeclaration { new_declaration, .. } => {
				message += &format!("{}\n{}", err, highlight_position_range(&new_declaration.name.position))
			},
			semantic_analyzer::Error::UndeclaredVariable { name } => {
				message += &format!("{}\n{}", err, highlight_position_range(&name.position))
			},
			semantic_analyzer::Error::TypeMismatch { position, .. } => {
				message += &format!("{}\n{}", err, highlight_position_range(position))
			},
			semantic_analyzer::Error::UndefinedFunctionCall { function_call } => {
				message += &format!("{}\n{}", err, highlight_position_range(&function_call.name.position))
			},
			semantic_analyzer::Error::ArgumentCountMismatch { function_call, .. } => {
				// TODO: Highlight position of `function_call.args` instead of `function_call.name.position`
				message += &format!("{}\n{}", err, highlight_position_range(&function_call.name.position))
			},
		}
	} else {
		message = err.to_string();
	}

	eprintln!("{}", message);
}

/// Highlights/underlines the affected position range in the source code line.
fn highlight_position_range(position: &SourcePositionRange) -> String {
	let affected_code = position.get_affected_lines();

	let mut output = String::new();

	for line_with_whitespaces in affected_code.lines() {
		let line = line_with_whitespaces.trim_start();
		let spaces_removed = line_with_whitespaces.len() - line.len();

		// Write source code line
		output.push_str(line);
		output.push('\n');

		// Write underline
		output.push_str(&" ".repeat(position.position.start.column - 1 - spaces_removed));
		let highlight_width = position.position.end.column - position.position.start.column + 1;
		output.push_str(&"^".repeat(highlight_width));
	}
	output
}
