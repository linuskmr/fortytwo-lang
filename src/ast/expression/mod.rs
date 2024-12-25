mod binary_expression;
mod binary_operator;
mod function_call;

pub use binary_expression::BinaryExpression;
pub use binary_operator::BinaryOperator;
pub use function_call::FunctionCall;

use crate::source::{PositionContainer, SourcePositionRange};

pub type Variable = PositionContainer<String>;

/// An expression produces a value.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	BinaryExpression(BinaryExpression),
	FunctionCall(FunctionCall),
	Number(Number),
	Variable(PositionContainer<String>),
}

impl Expression {
	pub fn source_position(&self) -> SourcePositionRange {
		match self {
			Expression::BinaryExpression(binary_expression) => binary_expression.source_position(),
			Expression::FunctionCall(function_call) => function_call.name.position.clone(),
			Expression::Number(number) => number.position.clone(),
			Expression::Variable(variable) => variable.position.clone(),
		}
	}
}

pub type Number = PositionContainer<NumberKind>;

#[derive(Debug, PartialEq, Clone)]
pub enum NumberKind {
	Int(i64),
	Float(f64),
}
