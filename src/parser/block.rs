use std::iter::Peekable;
use crate::ast::Instruction;
use crate::parser::helper;
use crate::parser::instruction::parse_instruction;
use crate::token::{Token, TokenKind};
use super::Result;


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