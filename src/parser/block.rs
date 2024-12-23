use std::iter::Peekable;

use super::Result;
use crate::{
	ast::Instruction,
	parser::{helper, instruction::parse_instruction},
	token::{Token, TokenKind},
};

pub fn parse_block(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Vec<Instruction>> {
	let mut block: Vec<Instruction> = Vec::new();
	helper::parse_opening_curly_parenthesis(tokens.next())?;
	while let Some(token) = tokens.peek() {
		if let TokenKind::ClosingCurlyBraces = **token {
			tokens.next(); // Consume TokenKind::ClosingParentheses
			break; // End of block
		}
		let instruction = parse_instruction(tokens)?;
		block.push(instruction);
	}
	Ok(block)
}
