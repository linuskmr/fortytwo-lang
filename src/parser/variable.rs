use std::iter::Peekable;
use crate::ast;
use crate::parser::{Error, expression, helper, variable};
use crate::source::PositionContainer;
use crate::token::{Token, TokenKind};
use super::Result;

pub fn parse_variable_declaration(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::statement::VariableDeclaration> {
	helper::parse_variable_declaration(tokens.next())?;
	let name = helper::parse_identifier(tokens.next())?;
	helper::parse_colon(tokens.next())?;
	let data_type = variable::parse_data_type(tokens)?;
	helper::parse_equal(tokens.next())?;
	let value = expression::parse_primary_expression(tokens)?;
	Ok(ast::statement::VariableDeclaration { name, data_type, value })
}

pub(crate) fn parse_data_type(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<PositionContainer<ast::statement::DataType>> {
	match tokens.next() {
		// Pointer type
		Some(Token { inner: TokenKind::Pointer, position }) => {
			// Recursively call parse_data_type to parse the type the pointer points to. This recursive calling
			// allows types like `ptr ptr int` to be parsed.
			let type_to_point_to = parse_data_type(tokens)?;
			Ok(PositionContainer {
				inner: ast::statement::DataType::Pointer(Box::new(type_to_point_to)),
				position,
			})
		},
		// Normal type
		Some(Token { inner: TokenKind::Identifier(type_str), position }) => {
			match ast::statement::BasicDataType::try_from(type_str.as_str()) {
				// Basic data type
				Ok(basic_data_type) => {
					Ok(PositionContainer {
						inner: ast::statement::DataType::Basic(basic_data_type),
						position,
					})
				},
				Err(_) => {
					// User-defined data type (struct)
					Ok(PositionContainer {
						inner: ast::statement::DataType::Struct(type_str),
						position,
					})
				}
			}
		},
		other => Err(Error::ExpectedToken {
			expected: TokenKind::Identifier(String::new()),
			found: other,
		}),
	}
}