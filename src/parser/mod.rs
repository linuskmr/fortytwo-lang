//! Analyzes [`Token`]s and build an [AST](ast).

mod block;
mod error;
mod expression;
mod function;
mod helper;
mod instruction;
mod struct_;
mod variable;

use std::{iter::Peekable, result};

pub use error::Error;
use try_match::try_match;

use crate::{
	ast,
	ast::{statement::BasicDataType, Instruction, Node},
	parser::{
		function::{parse_extern_function_declaration, parse_function_call, parse_function_definition},
		struct_::parse_struct_definition,
	},
	source::PositionContainer,
	token::{Token, TokenKind},
};

pub type Result<T> = std::result::Result<T, Error>;

/// Analyzes [`Token`]s and builds an [AST](ast).
pub struct Parser<T>
where
	T: Iterator<Item = Token>,
{
	tokens: Peekable<T>,
}

impl<T> Parser<T>
where
	T: Iterator<Item = Token>,
{
	pub fn new(tokens: T) -> Self {
		Self { tokens: tokens.peekable() }
	}
}

fn parse_top_level_node(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Result<Node>> {
	let token = tokens.peek()?;
	match **token {
		TokenKind::Def => Some(parse_function_definition(tokens).map(Node::Function)),
		TokenKind::Extern => Some(parse_extern_function_declaration(tokens).map(Node::FunctionPrototype)),
		TokenKind::Struct => Some(parse_struct_definition(tokens).map(Node::Struct)),
		TokenKind::Comment(ref comment) => {
			tracing::warn!("Skipping {}", token);
			tokens.next();
			parse_top_level_node(tokens)
		},
		_ => Some(Err(Error::IllegalToken { token: Some(tokens.next()?), context: "top level node" })),
	}
}

impl<T> Iterator for Parser<T>
where
	T: Iterator<Item = Token>,
{
	type Item = Result<Node>;

	fn next(&mut self) -> Option<Self::Item> {
		parse_top_level_node(&mut self.tokens)
	}
}
