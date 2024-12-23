use std::iter::Peekable;

use super::Result;
use crate::{
	ast,
	ast::{
		expression::{BinaryOperator, NumberKind},
		Expression,
	},
	parser::{function::parse_function_call, helper, helper::parse_operator, Error},
	source::PositionContainer,
	token::{Token, TokenKind},
};

pub(crate) fn parse_primary_expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Expression> {
	match tokens.peek() {
		Some(Token { inner: TokenKind::Identifier(_), .. }) => Ok(parse_identifier_expression(tokens)?),
		Some(Token { inner: TokenKind::Float(_), .. }) => Ok(ast::Expression::Number(parse_float(tokens)?)),
		Some(Token { inner: TokenKind::Int(_), .. }) => Ok(ast::Expression::Number(parse_int(tokens)?)),
		Some(Token { inner: TokenKind::OpeningParentheses, .. }) => Ok(parse_parentheses(tokens)?),
		other => Err(Error::IllegalToken { token: other.cloned(), context: "expression" }),
	}
}

pub fn parse_float(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<PositionContainer<NumberKind>> {
	match tokens.next() {
		Some(Token { inner: TokenKind::Float(float), position }) => {
			Ok(PositionContainer::new(NumberKind::Float(float), position))
		},
		Some(Token { inner: TokenKind::Int(int), position }) => {
			Ok(PositionContainer::new(NumberKind::Int(int), position))
		},
		other => Err(Error::ExpectedToken { expected: TokenKind::Float(0.0), found: other }),
	}
}

pub fn parse_int(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<PositionContainer<NumberKind>> {
	match tokens.next() {
		Some(Token { inner: TokenKind::Int(int), position }) => {
			Ok(PositionContainer::new(NumberKind::Int(int), position))
		},
		other => Err(Error::ExpectedToken { expected: TokenKind::Int(0), found: other }),
	}
}

pub fn parse_identifier_expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Expression> {
	let identifier = helper::parse_identifier(tokens.next())?;
	match tokens.peek() {
		Some(Token { inner: TokenKind::OpeningParentheses, .. }) => {
			Ok(ast::Expression::FunctionCall(parse_function_call(tokens, identifier)?))
		},
		_ => Ok(ast::Expression::Variable(identifier)),
	}
}

pub fn parse_parentheses(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::expression::Expression> {
	helper::parse_opening_parenthesis(tokens.next())?;
	let expression = parse_binary_expression(tokens)?;
	helper::parse_closing_parenthesis(tokens.next())?;
	Ok(expression)
}

pub(crate) fn parse_binary_expression(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::expression::Expression> {
	let lhs = parse_primary_expression(tokens)?;
	parse_binary_expression_rhs(lhs, None, tokens)
}

fn parse_binary_expression_rhs(
	lhs: Expression,
	min_operator: Option<&BinaryOperator>,
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::expression::Expression> {
	let mut lhs: ast::Expression = lhs;
	loop {
		// Read the operator after lhs and before rhs
		let operator = match parse_operator(tokens.peek().cloned()) {
			// Found an operator
			Ok(operator) => operator,
			// No operator found
			Err(_) => return Ok(lhs),
		};
		// Consume operator
		tokens.next();

		// Parse the primary expression after the operator as rhs
		let mut rhs = parse_primary_expression(tokens)?;

		// Inspect the next operator after rhs. If it has a higher precedence than the current operator,
		// let rhs be the result of a recursive call to parse_binary_expression_rhs with rhs as lhs.
		if let Ok(next_operator) = parse_operator(tokens.peek().cloned()) {
			if next_operator > operator {
				rhs = parse_binary_expression_rhs(rhs, Some(&next_operator), tokens)?;
			}
		}

		// Merge lhs and rhs into a new lhs
		lhs = ast::Expression::BinaryExpression(ast::expression::BinaryExpression {
			lhs: Box::new(lhs),
			rhs: Box::new(rhs),
			operator,
		});
	}
	Ok(lhs)
}
