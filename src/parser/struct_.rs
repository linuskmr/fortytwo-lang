use std::iter::Peekable;

use super::Result;
use crate::{
	ast,
	parser::{helper, variable::parse_data_type},
	token::{Token, TokenKind},
};

pub(crate) fn parse_struct_definition(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::struct_::Struct> {
	helper::parse_struct(tokens.next())?;
	let name = helper::parse_identifier(tokens.next())?;
	helper::parse_opening_curly_parenthesis(tokens.next())?;
	let mut fields: Vec<ast::struct_::Field> = Vec::new();
	while let Some(token) = tokens.peek() {
		if let TokenKind::ClosingCurlyBraces = **token {
			tokens.next(); // Consume TokenKind::ClosingParentheses
			break; // End of block
		}
		let field = parse_field(tokens)?;
		fields.push(field);
	}
	Ok(ast::struct_::Struct { name, fields })
}

pub(crate) fn parse_field(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::struct_::Field> {
	let name = helper::parse_identifier(tokens.next())?;
	helper::parse_colon(tokens.next())?;
	let data_type = parse_data_type(tokens)?;
	Ok(ast::struct_::Field { name, data_type })
}
