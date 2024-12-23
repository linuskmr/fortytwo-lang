use std::{fmt, ops::RangeInclusive, sync::Arc};

use crate::source::Source;

/// Line and column in source code.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
pub struct Position {
	pub line: usize,
	pub column: usize,
	pub offset: usize,
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", self.line, self.column)
	}
}

impl Default for Position {
	fn default() -> Self {
		Position { line: 1, column: 1, offset: 0 }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_display() {
		let position = Position { line: 42, column: 5, offset: 1337 };
		assert_eq!(position.to_string(), "42:5")
	}
}
