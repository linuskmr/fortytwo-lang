use super::Expression;
use crate::source::PositionContainer;
use std::fmt;
use std::ops::Deref;

/// A function call, i.e. the execution of a [`FunctionDefinition`](crate::ast::FunctionDefinition) with concrete parameters.
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
	/// The name of the function to be called.
	pub name: PositionContainer<String>,
	/// The parameters to invoke the called function with.
	pub params: Vec<Expression>,
}

impl fmt::Display for FunctionCall {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "`{}(...)` at {}", self.name.deref(), self.name.position)
	}
}
