use crate::ast::statement::DataType;
use crate::source::PositionContainer;

/// A function prototype, i.e. the header of the function. It consists of the function name and arguments.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Struct {
	/// The name of the struct.
	pub name: PositionContainer<String>,
	/// The fields of the struct.
	pub fields: Vec<Field>,
}

/// A struct field consists of a name and a type that specify a field of a struct.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Field {
	/// The name of the struct field.
	pub name: PositionContainer<String>,
	/// The type of the field, e.g. a int, a struct or a pointer.
	pub data_type: PositionContainer<DataType>,
}