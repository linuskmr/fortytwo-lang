use std::{
	fmt,
	fmt::Formatter,
	hash::{Hash, Hasher},
	ops::Deref,
};

use crate::{ast::statement::DataType, source::PositionContainer};

/// Stores the name and type of a currently in-scope variable in the call stack.
///
/// Two variables are considered equal if they have the same name.
/// If two variables with the same name but different types are declared, this results in a *name conflict*.
#[derive(Debug)]
pub struct Variable {
	/// The name of the variable and the position of the declaration.
	pub name: PositionContainer<String>,
	/// The type of the variable.
	pub type_: DataType,
}

impl fmt::Display for Variable {
	/// Formats the variable as `name: type`.
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.name.deref(), self.type_)
	}
}

impl Hash for Variable {
	fn hash<H: Hasher>(&self, state: &mut H) {
		// Hashing `self.name` includes the position, leading to a different hash for variables
		// of the same name.
		self.name.value.hash(state);
	}
}

impl PartialEq for Variable {
	fn eq(&self, other: &Self) -> bool {
		// Comparing `self.name` includes the position, leading to a two variables not being
		// equal when they have the same name.
		self.name.value == other.name.value
	}
}

impl Eq for Variable {}
