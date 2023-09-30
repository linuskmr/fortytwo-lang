//! Analyzes [`Token`]s and build an [AST](ast).

mod block;
mod error;
mod expression;
mod function;
mod helper;
mod instruction;
mod struct_;
mod variable;

use crate::ast;
use crate::ast::{Instruction, Node};
use crate::token::{Token, TokenKind};
use std::iter::Peekable;
use std::result;
use try_match::try_match;

use crate::ast::statement::BasicDataType;
use crate::parser::function::parse_function_definition;
use crate::parser::struct_::parse_struct_definition;
use crate::source::PositionContainer;
pub use error::Error;

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
		Self {
			tokens: tokens.peekable(),
		}
	}
}

fn parse_top_level_node(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Option<Result<Node>> {
	Some(match **tokens.peek()? {
		TokenKind::Def => parse_function_definition(tokens).map(Node::Function),
		TokenKind::Struct => parse_struct_definition(tokens).map(Node::Struct),
		_ => Err(Error::IllegalToken {
			token: Some(tokens.next()?),
			context: "top level node",
		}),
	})
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
