use std::fmt;
use std::ops::RangeInclusive;
use std::sync::Arc;
use crate::source::Source;


/// Line and column in source code.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy, Default)]
pub struct Position {
	pub line: usize,
	pub column: usize,
}

impl Position {
	pub fn user_line(&self) -> usize {
		self.line + 1
	}

	pub fn user_column(&self) -> usize {
		self.column + 1
	}
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", self.user_line(), self.user_column())
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_display() {
		let position = Position {
			line: 42,
			column: 5,
		};
		assert_eq!(position.to_string(), "43:6")
	}
}