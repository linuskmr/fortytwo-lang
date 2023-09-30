use crate::ast::function_argument::FunctionArgument;
use crate::ast::statement::DataType;
use crate::source::PositionContainer;
use std::fmt::Display;

/// The header of the function i.e. function name and arguments, but not the body.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FunctionPrototype {
	/// The name of the function.
	pub name: PositionContainer<String>,
	/// The arguments for the function.
	pub args: Vec<FunctionArgument>,
	/// Return type is what this function returns.
	pub return_type: Option<PositionContainer<DataType>>,
}
