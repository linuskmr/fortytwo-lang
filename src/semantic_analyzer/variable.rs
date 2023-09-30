use crate::ast::statement::DataType;
use crate::source::PositionContainer;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// TODO
///
/// Two variables are considered equal if they have the same name.
/// If two variables with the same name but different types are declared, this results in a *name conflict*.
#[derive(Debug)]
pub struct Variable {
	pub name: PositionContainer<String>,
	pub type_: DataType,
}

impl fmt::Display for Variable {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.name.deref(), self.type_)
	}
}

impl Hash for Variable {
	fn hash<H: Hasher>(&self, state: &mut H) {
		// Hashing `self.name` includes the position, leading to a different hash for variables
		// of the same name.
		self.name.inner.hash(state);
	}
}

impl PartialEq for Variable {
	fn eq(&self, other: &Self) -> bool {
		// Comparing `self.name` includes the position, leading to a two variables not being
		// equal when they have the same name.
		self.name.inner == other.name.inner
	}
}

impl Eq for Variable {}
