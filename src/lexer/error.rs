use crate::source::{PositionContainer, Symbol};
use thiserror::Error;

/// Lexer errors.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum Error {
	#[error("Unknown symbol {0}")]
	UnknownSymbol(Symbol),
	#[error("Illegal symbol {}", .0.as_ref().map(|s| s.to_string()).unwrap_or("None".to_owned()))]
	IllegalSymbol(Option<Symbol>),
	#[error("Could not parse number {0}")]
	ParseNumberError(PositionContainer<String>),
}
