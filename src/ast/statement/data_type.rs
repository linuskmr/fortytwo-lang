use std::fmt;

use super::basic_data_type::BasicDataType;
use crate::source::PositionContainer;

/// A data type is either basic, a struct, or a pointer to a data type.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum DataType {
	/// A basic data type like int and float.
	Basic(BasicDataType),
	/// A user defined struct with custom name.
	Struct(String),
	/// A Pointer to a data type.
	Pointer(Box<PositionContainer<DataType>>),
}

impl fmt::Display for DataType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			DataType::Basic(basic_data_type) => write!(f, "{}", basic_data_type),
			DataType::Struct(struct_name) => write!(f, "{}", struct_name),
			DataType::Pointer(pointer) => write!(f, "ptr {}", pointer.value),
		}
	}
}
