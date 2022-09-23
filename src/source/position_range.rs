use std::fmt;
use std::ops::RangeInclusive;
use std::sync::Arc;
use crate::source::{Position, Source};

/// Position in the source code ranging from start to end (both inclusive).
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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
			},
			end: Position {
				line: 43,
				column: 1,
			}
		};
		assert_eq!(position.to_string(), "43:6-44:2")
	}
}