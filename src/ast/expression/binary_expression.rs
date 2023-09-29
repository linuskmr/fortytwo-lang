use super::BinaryOperator;
use crate::ast::Expression;
use crate::source::PositionContainer;
use std::fmt::Display;

/// A binary expression of the form `lhs op rhs` like `40 + 2`.
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
	/// The left hand side.
	pub lhs: Box<Expression>,
	/// The operator connecting `lhs` and `rhs`.
	pub operator: PositionContainer<BinaryOperator>,
	/// The right hand side.
	pub rhs: Box<Expression>,
}
