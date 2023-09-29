use super::basic_data_type::BasicDataType;
use crate::source::PositionContainer;
use std::fmt::Display;

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
