use crate::ast::statement::DataType;
use crate::source::PositionContainer;

/// A function argument consists of a name and a type that specify an argument of a function in its function prototype.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct FunctionArgument {
	/// The name of the function argument.
	pub name: PositionContainer<String>,
	/// The type of the argument, e.g. a int, a struct or a pointer.
	pub data_type: PositionContainer<DataType>,
}