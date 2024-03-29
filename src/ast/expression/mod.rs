mod binary_expression;
mod binary_operator;
mod function_call;

pub use binary_expression::BinaryExpression;
pub use binary_operator::BinaryOperator;
pub use function_call::FunctionCall;
use std::fmt::Display;

use crate::source::PositionContainer;

pub type Variable = PositionContainer<String>;

/// An expression produces a value.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	BinaryExpression(BinaryExpression),
	FunctionCall(FunctionCall),
	Number(Number),
	Variable(PositionContainer<String>),
}

pub type Number = PositionContainer<NumberKind>;

#[derive(Debug, PartialEq, Clone)]
pub enum NumberKind {
	Int(i64),
	Float(f64),
}
