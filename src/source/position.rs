use std::{fmt, ops::RangeInclusive, sync::Arc};

use crate::source::Source;

/// Line and column in source code.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
pub struct Position {
	/// Line in the source code file.
	pub line: usize,
	/// Column in the [line](Self::line) in the source code file.
	pub column: usize,
	/// Byte offset from the start of the source code file.
	pub offset: usize,
}

impl fmt::Display for Position {
	/// ```
	/// use fortytwolang::source::Position;
	/// let position = Position { line: 42, column: 5, offset: 1337 };
	/// assert_eq!(position.to_string(), "42:5");
	/// ```
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", self.line, self.column)
	}
}

impl Default for Position {
	fn default() -> Self {
		Position { line: 1, column: 1, offset: 0 }
	}
}
