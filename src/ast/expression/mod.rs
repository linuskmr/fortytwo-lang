mod binary_expression;
mod binary_operator;
mod function_call;

pub use binary_expression::BinaryExpression;
pub use binary_operator::BinaryOperator;
pub use function_call::FunctionCall;
use std::fmt::Display;

use crate::source::PositionContainer;

pub type Number = PositionContainer<f64>;
pub type Variable = PositionContainer<String>;

/// Binary expression, function call, number or variable.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    FunctionCall(FunctionCall),
    Number(Number),
    Variable(PositionContainer<String>),
}
