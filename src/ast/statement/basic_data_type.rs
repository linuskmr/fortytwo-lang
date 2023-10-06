use std::fmt;

/// A basic data type is a type with hardware support like int and float.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BasicDataType {
	/// A integer number, like 42
	Int,
	/// A floating point number like 4.2
	Float,
}

impl TryFrom<&str> for BasicDataType {
	type Error = ();

	/// Converts a data type (as string) to a [BasicDataType] enum. If `data_type` does not match any [BasicDataType],
	/// this method will return Err.
	fn try_from(data_type: &str) -> Result<Self, Self::Error> {
		match data_type {
			"int" => Ok(BasicDataType::Int),
			"float" => Ok(BasicDataType::Float),
			_ => Err(()), // No basic data type with this name
		}
	}
}

impl fmt::Display for BasicDataType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			BasicDataType::Int => write!(f, "int"),
			BasicDataType::Float => write!(f, "float"),
		}
	}
}
