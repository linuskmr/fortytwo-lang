use std::fmt;
use std::sync::Arc;
use crate::source::position_range::PositionRange;
use crate::source::Source;

/// Position in the source code ranging from start to end (both inclusive).
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SourcePositionRange {
	pub source: Arc<Source>,
	pub position: PositionRange,
}

impl SourcePositionRange {
	pub fn get_affected_lines(&self) -> Vec<String> {
		self.source.text[self.position.start.line..=self.position.end.line]
			.iter()
			.map(|line| line.iter().collect())
			.collect()
	}
}

impl fmt::Display for SourcePositionRange {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", self.source.name, self.position.start)
	}
}

#[cfg(test)]
mod tests {
	use crate::source::Position;
	use super::*;

	#[test]
	fn test_display() {
		let position = SourcePositionRange {
			source: Arc::new(Source::new(
				"file.name".to_owned(),
				"text...".to_owned(),
			)),
			position: PositionRange {
				start: Position {
					line: 42,
					column: 5,
				},
				end: Position {
					line: 43,
					column: 1,
				}
			}
		};
		assert_eq!(position.to_string(), "file.name:43:6")
	}
}