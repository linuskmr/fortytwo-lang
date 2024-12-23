use super::BinaryOperator;
use crate::ast::Expression;
use crate::source::{PositionContainer, PositionRange, SourcePositionRange};
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

impl BinaryExpression {
	pub fn source_position(&self) -> SourcePositionRange {
		let mut position = self.lhs.source_position();
		position.position.end = self.rhs.source_position().position.end;
		position
	}
}
