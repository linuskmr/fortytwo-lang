use super::{Error, Result};
use crate::ast;
use crate::ast::Expression;
use crate::parser::block::parse_block;
use crate::parser::expression::parse_primary_expression;
use crate::parser::{block, helper, variable};
use crate::source::PositionContainer;
use crate::token::{Token, TokenKind};
use std::iter::Peekable;

pub fn parse_function_definition(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::statement::FunctionDefinition> {
	tokens.next(); // Consume TokenKind::FunctionDefinition
	let prototype = parse_function_prototype(tokens)?;
	let body = parse_block(tokens)?;
	Ok(ast::statement::FunctionDefinition { prototype, body })
}

pub fn parse_extern_function_declaration(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::statement::FunctionPrototype> {
	tokens.next(); // Consume TokenKind::Extern
	parse_function_prototype(tokens)
}

fn parse_function_prototype(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::statement::FunctionPrototype> {
	let name = helper::parse_identifier(tokens.next())?;
	let args = parse_function_argument_list(tokens)?;
	let return_type = parse_function_prototype_return_type(tokens)?;
	Ok(ast::statement::FunctionPrototype {
		name,
		args,
		return_type,
	})
}

fn parse_function_argument_list(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<ast::statement::FunctionArgument>> {
	helper::parse_opening_parenthesis(tokens.next())?;
	let mut arguments: Vec<ast::statement::FunctionArgument> = Vec::new();

	// Check whether the argument list is empty, i.e. whether the next token is a closing parenthesis
	if let Some(Token {
		inner: TokenKind::ClosingParentheses,
		..
	}) = tokens.peek()
	{
		tokens.next(); // Consume the closing parenthesis
		return Ok(arguments);
	}

	// Collect all arguments until closing parentheses
	loop {
		let name = helper::parse_identifier(tokens.next())?;
		helper::parse_colon(tokens.next())?;
		let data_type = variable::parse_data_type(tokens)?;
		arguments.push(ast::statement::FunctionArgument { name, data_type });
		match tokens.peek() {
			Some(Token {
				inner: TokenKind::Comma,
				..
			}) => {
				tokens.next(); // Consume the comma
			}
			_ => break, // No comma after this argument, so this is the last argument
		}
	}
	helper::parse_closing_parenthesis(tokens.next())?;
	Ok(arguments)
}

fn parse_function_prototype_return_type(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Option<PositionContainer<ast::statement::DataType>>> {
	match tokens.peek() {
		// No return type specified
		Some(Token {
			inner: TokenKind::OpeningCurlyBraces,
			..
		}) => Ok(None),
		// Return type specified
		Some(Token {
			inner: TokenKind::Colon,
			..
		}) => {
			tokens.next(); // Consume TokenKind::Colon
			let data_type = variable::parse_data_type(tokens)?;
			Ok(Some(data_type))
		}
		_ => Ok(None),
	}
}

pub(crate) fn parse_function_call(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	identifier: PositionContainer<String>,
) -> Result<ast::expression::FunctionCall> {
	Ok(ast::expression::FunctionCall {
		name: identifier,
		params: parse_function_parameters(tokens)?,
	})
}

fn parse_function_parameters(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<Expression>> {
	helper::parse_opening_parenthesis(tokens.next())?;
	let mut parameters: Vec<Expression> = Vec::new();

	// Check whether the parameter list is empty, i.e. whether the next token is a closing parenthesis
	if let Some(Token {
		inner: TokenKind::ClosingParentheses,
		..
	}) = tokens.peek()
	{
		tokens.next(); // Consume the closing parenthesis
		return Ok(parameters);
	}

	// Collect all parameters until closing parentheses
	loop {
		let parameter = parse_primary_expression(tokens)?;
		parameters.push(parameter);
		match tokens.peek() {
			Some(Token {
				inner: TokenKind::Comma,
				..
			}) => {
				tokens.next(); // Consume the comma
			}
			_ => break, // No comma after this parameter, so this is the last parameter
		}
	}

	helper::parse_closing_parenthesis(tokens.next())?;
	Ok(parameters)
}
