use std::iter::Peekable;
use crate::ast;
use crate::ast::Statement;
use crate::parser::{Error, expression, helper};
use crate::parser::block::parse_block;
use crate::parser::expression::{parse_identifier_expression, parse_number, parse_parentheses};
use crate::parser::function::parse_function_call;
use crate::parser::variable::parse_variable_declaration;
use crate::source::PositionContainer;
use crate::token::{Token, TokenKind};
use super::Result;


pub fn parse_instruction(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Instruction> {
	match tokens.peek() {
		Some(Token { inner: TokenKind::Identifier(_), .. }) => {
			Ok(parse_identifier_instruction(tokens)?)
		},
		Some(Token { inner: TokenKind::Number(_), .. }) => {
			Ok(ast::Instruction::Expression(ast::Expression::Number(parse_number(tokens)?)))
		},
		Some(Token { inner: TokenKind::OpeningParentheses, .. }) => {
			Ok(ast::Instruction::Expression(parse_parentheses(tokens)?))
		},
		Some(Token { inner: TokenKind::If, .. }) => {
			Ok(ast::Instruction::IfElse(Box::new(parse_if_else(tokens)?)))
		},
		Some(Token { inner: TokenKind::While, .. }) => {
			Ok(ast::Instruction::WhileLoop(Box::new(parse_while_loop(tokens)?)))
		},
		Some(Token { inner: TokenKind::Var, .. }) => {
			Ok(ast::Instruction::Statement(Statement::VariableDeclaration(parse_variable_declaration(tokens)?)))
		},
		other => {
			return Err(Error::IllegalToken {
				token: other.cloned(),
				context: "instruction",
			})
		}
	}
}

pub fn parse_if_else(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::IfElse> {
	helper::parse_if(tokens.next())?;
	let condition = expression::parse_binary_expression(tokens)?;
	let if_true = parse_block(tokens)?;
	let if_false = match tokens.peek() {
		Some(Token { inner: TokenKind::Else, .. }) => {
			tokens.next(); // Consume the TokenKind::Else
			let block = parse_block(tokens)?;
			block
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
		Some(Token { inner: TokenKind::OpeningParentheses, .. }) => {
			Ok(ast::Instruction::Expression(ast::Expression::FunctionCall(parse_function_call(tokens, identifier)?)))
		},
		Some(Token { inner: TokenKind::Equal, .. }) => {
			tokens.next(); // Consume the TokenKind::Equal
			Ok(ast::Instruction::Statement(ast::Statement::VariableAssignment(ast::statement::VariableAssignment {
				name: identifier,
				value: expression::parse_binary_expression(tokens)?,
			})))
		},
		_ => {
			Ok(ast::Instruction::Expression(ast::Expression::Variable(identifier)))
		},
	}
}