use std::{fs, path::Path, sync::Arc};

use anyhow::Context;
use lexer::Lexer;
use parser::Parser;
use semantic_analyzer::{SymbolTable, TypeChecker};
use source::Source;
use token::Token;

pub mod ast;
pub mod emitter;
pub mod lexer;
pub mod parser;
pub mod semantic_analyzer;
pub mod source;
pub mod token;

/// Combines lexer, parser, and semantic analysis into a single function.
pub fn compiler_pipeline(path: &Path) -> anyhow::Result<Vec<ast::Node>> {
	let content = fs::read_to_string(path).context(format!("Reading FTL source file `{:?}`", path))?;

	let source = Arc::new(Source::new(path.to_str().unwrap().to_string(), content));
	let lexer = Lexer::new(source.iter());
	let tokens = lexer.collect::<Result<Vec<Token>, lexer::Error>>().context("Lexing error")?;

	let parser = Parser::new(tokens.into_iter());
	let ast_nodes = parser.collect::<Result<Vec<_>, _>>().context("Parser error")?;
	tracing::trace!("AST parsed: {:#?}", ast_nodes);

	let symbol_table = SymbolTable::global_symbol_scan(ast_nodes.iter()).context("Global symbol scan error")?;
	TypeChecker::type_check(symbol_table, ast_nodes.iter()).context("Type checking error")?;

	Ok(ast_nodes)
}
