use crate::ast::statement::DataType;
use crate::ast::function_argument::FunctionArgument;
use crate::source::PositionContainer;

/// A function prototype, i.e. the header of the function. It consists of the function name and arguments.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct FunctionPrototype {
	/// The name of the function.
	pub name: PositionContainer<String>,
	/// The arguments for the function.
	pub args: Vec<FunctionArgument>,
	/// Return type is what this function returns.
	pub return_type: Option<PositionContainer<DataType>>,
}