mod function_call;
mod binary_expression;
mod binary_operator;


pub use function_call::FunctionCall;
pub use binary_expression::BinaryExpression;
pub use binary_operator::BinaryOperator;

use crate::source::PositionContainer;


/// Binary expression, function call, number or variable.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	BinaryExpression(BinaryExpression),
	FunctionCall(FunctionCall),
	Number(PositionContainer<f64>),
	Variable(PositionContainer<String>),
}