use crate::source::position_range::PositionRange;
use crate::source::Source;
use std::fmt;
use std::sync::Arc;

/// Position in the source code ranging from start to end (both inclusive).
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SourcePositionRange {
	pub source: Arc<Source>,
	pub position: PositionRange,
}

impl SourcePositionRange {
	pub fn get_affected_lines(&self) -> String {
		let source_string = self.source.text.iter().collect::<String>();
		let lines: Vec<&str> = source_string.lines().collect();
		lines[self.position.start.line - 1..=self.position.end.line - 1].join("\n")
	}

	pub fn get_affected_code(&self) -> String {
		self.source.text[self.position.start.offset..=self.position.end.offset]
			.iter()
			.collect::<String>()
	}
}

impl fmt::Display for SourcePositionRange {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.source.name, self.position.start)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::source::Position;

	#[test]
	fn test_display() {
		let position = SourcePositionRange {
			source: Arc::new(Source::new("file.name".to_owned(), "text...".to_owned())),
			position: PositionRange {
				start: Position {
					line: 42,
					column: 5,
					offset: 1337,
				},
				end: Position {
					line: 43,
					column: 1,
					offset: 1340,
				},
			},
		};
		assert_eq!(position.to_string(), "file.name:42:5")
	}
}
