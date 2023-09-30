use crate::ast::statement::DataType;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Variable {
	pub name: String,
	pub type_: DataType,
}

impl fmt::Display for Variable {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.name, self.type_)
	}
}

impl Hash for Variable {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl PartialEq for Variable {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Variable {}
