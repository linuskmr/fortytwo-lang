use std::fmt;
use std::ops::Deref;
use crate::source::Position;
use crate::source::source_position::SourcePositionRange;

/// Wrapper for values inside source code with position information.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionContainer<T> {
	pub position: SourcePositionRange,
	inner: T,
}

impl<T> PositionContainer<T> {
	pub fn new(value: T, position: SourcePositionRange) -> Self {
		Self { inner: value, position }
	}
}

impl<T> Deref for PositionContainer<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T: fmt::Display> fmt::Display for PositionContainer<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "'{}' at {}", self.inner, self.position)
	}
}


#[cfg(test)]
mod tests {
	use std::sync::Arc;
	use crate::source::position_range::PositionRange;
	use crate::source::Source;
	use super::*;

	/// Tests that a PositionContainer can be dereferences to its inner value.
	///
	/// More a compile-time check than a runtime test.
	#[test]
	fn test_deref() {
		/// Potential expression struct used somewhere in the AST.
		#[derive(PartialEq, Debug)]
		struct Number(f64);

		let expression = PositionContainer::new(
			Number(1.0),
			SourcePositionRange {
				source: Arc::new(Source::new(
					"file.name".to_owned(),
					"content".to_owned(),
				)),
				position: PositionRange {
					start: Position {
						line: 0,
						column: 0
					},
					end: Position {
						line: 0,
						column: 0
					}
				},
			}
		);

		assert_eq!(*expression, Number(1.0), "Deref to inner value failed");
	}
}