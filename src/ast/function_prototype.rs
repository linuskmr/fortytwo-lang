use std::fmt::Display;

use crate::{
	ast::{function_argument::FunctionArgument, statement::DataType},
	source::PositionContainer,
};

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
