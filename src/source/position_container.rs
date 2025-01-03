use std::{fmt, ops::Deref};

use crate::source::source_position::SourcePositionRange;

/// Wrapper for values inside source code with position information.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionContainer<T> {
	/// Where the [`value`](Self::value) is located in the source code.
	pub position: SourcePositionRange,
	/// The value that is wrapped with [position information](Self::position).
	pub value: T,
}

impl<T> PositionContainer<T> {
	pub fn new(value: T, position: SourcePositionRange) -> Self {
		Self { value, position }
	}
}

impl<T> Deref for PositionContainer<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T: fmt::Display> fmt::Display for PositionContainer<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "'{}' at {}", self.value, self.position)
	}
}

impl<T: PartialOrd> PartialOrd for PositionContainer<T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.value.partial_cmp(&other.value)
	}
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use super::*;
	use crate::source::{position_range::PositionRange, Source};

	/// Tests that a [`PositionContainer`] can be dereferences to its inner value.
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
				source: Arc::new(Source::new("file.name".to_owned(), "content".to_owned())),
				position: PositionRange::default(),
			},
		);

		assert_eq!(*expression, Number(1.0), "Deref to inner value failed");
	}
}
