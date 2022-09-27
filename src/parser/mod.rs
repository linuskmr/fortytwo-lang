mod error;
mod helper;
mod function;
mod block;
mod struct_;
mod instruction;
mod expression;
mod variable;

use std::iter::Peekable;
use std::result;
use try_match::try_match;
use crate::ast::{AstNode, Instruction};
use crate::ast;
use crate::token::{Token, TokenKind};

pub use error::Error;
use crate::ast::statement::BasicDataType;
use crate::parser::function::parse_function_definition;
use crate::parser::struct_::parse_struct_definition;
use crate::source::PositionContainer;

pub type Result<T> = std::result::Result<T, Error>;


pub struct Parser<T>
where
	T: Iterator<Item = Token>,
{
	tokens: Peekable<T>,
}

impl<T> Parser<T>
where
	T: Iterator<Item = Token>
{
	pub fn new(tokens: T) -> Self {
		Self {
			tokens: tokens.peekable(),
		}
	}
}

fn parse_top_level_node(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Result<AstNode>> {
	Some(match **tokens.peek()? {
		TokenKind::Def => {
			parse_function_definition(tokens)
				.map(AstNode::Function)
		},
		TokenKind::Struct => {
			parse_struct_definition(tokens)
				.map(AstNode::Struct)
		},
		_ => Err(Error::IllegalToken {
			token: Some(tokens.next()?),
			context: "top level node",
		}),
	})
}



impl<T> Iterator for Parser<T>
where
	T: Iterator<Item = Token>
{
	type Item = Result<AstNode>;

	fn next(&mut self) -> Option<Self::Item> {
		parse_top_level_node(&mut self.tokens)
	}
}