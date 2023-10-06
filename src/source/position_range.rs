use crate::source::{Position, Source};
use std::fmt;
use std::ops::RangeInclusive;
use std::sync::Arc;

/// Position in the source code ranging from start to end (both inclusive).
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub struct PositionRange {
	pub start: Position,
	pub end: Position,
}

impl fmt::Display for PositionRange {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}-{}", self.start, self.end)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_display() {
		let position = PositionRange {
			start: Position {
				line: 42,
				column: 5,
				offset: 1337,
			},
			end: Position {
				line: 43,
				column: 1,
				offset: 1338,
			},
		};
		assert_eq!(position.to_string(), "42:5-43:1")
	}
}
