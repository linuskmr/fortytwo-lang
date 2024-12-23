use std::iter::Peekable;

use super::Result;
use crate::{
	ast,
	ast::Statement,
	parser::{
		block::parse_block,
		expression,
		expression::{parse_float, parse_identifier_expression, parse_int, parse_parentheses},
		function::parse_function_call,
		helper,
		variable::parse_variable_declaration,
		Error,
	},
	source::PositionContainer,
	token::{Token, TokenKind},
};

pub fn parse_instruction(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Instruction> {
	match tokens.peek() {
		Some(Token { value: TokenKind::Identifier(_), .. }) => Ok(parse_identifier_instruction(tokens)?),
		Some(Token { value: TokenKind::Float(_), .. }) => {
			Ok(ast::Instruction::Expression(ast::Expression::Number(parse_float(tokens)?)))
		},
		Some(Token { value: TokenKind::Int(_), .. }) => {
			Ok(ast::Instruction::Expression(ast::Expression::Number(parse_int(tokens)?)))
		},
		Some(Token { value: TokenKind::OpeningParentheses, .. }) => {
			Ok(ast::Instruction::Expression(parse_parentheses(tokens)?))
		},
		Some(Token { value: TokenKind::If, .. }) => Ok(ast::Instruction::IfElse(Box::new(parse_if_else(tokens)?))),
		Some(Token { value: TokenKind::While, .. }) => {
			Ok(ast::Instruction::WhileLoop(Box::new(parse_while_loop(tokens)?)))
		},
		Some(Token { value: TokenKind::Var, .. }) => {
			Ok(ast::Instruction::Statement(Statement::VariableDeclaration(parse_variable_declaration(tokens)?)))
		},
		other => Err(Error::IllegalToken { token: other.cloned(), context: "instruction" }),
	}
}

pub fn parse_if_else(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::IfElse> {
	helper::parse_if(tokens.next())?;
	let condition = expression::parse_binary_expression(tokens)?;
	let if_true = parse_block(tokens)?;
	let if_false = match tokens.peek() {
		Some(Token { value: TokenKind::Else, .. }) => {
			tokens.next(); // Consume the TokenKind::Else
			parse_block(tokens)?
		},
		_ => Vec::new(),
	};

	Ok(ast::IfElse { condition, if_true, if_false })
}

pub fn parse_while_loop(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::WhileLoop> {
	helper::parse_while(tokens.next())?;
	let condition = expression::parse_binary_expression(tokens)?;
	let body = parse_block(tokens)?;
	Ok(ast::WhileLoop { condition, body })
}

pub fn parse_identifier_instruction(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Instruction> {
	let identifier = helper::parse_identifier(tokens.next())?;
	match tokens.peek() {
		Some(Token { value: TokenKind::OpeningParentheses, .. }) => {
			Ok(ast::Instruction::Expression(ast::Expression::FunctionCall(parse_function_call(tokens, identifier)?)))
		},
		Some(Token { value: TokenKind::Equal, .. }) => {
			tokens.next(); // Consume the TokenKind::Equal
			Ok(ast::Instruction::Statement(ast::Statement::VariableAssignment(ast::statement::VariableAssignment {
				name: identifier,
				value: expression::parse_binary_expression(tokens)?,
			})))
		},
		_ => Ok(ast::Instruction::Expression(ast::Expression::Variable(identifier))),
	}
}
