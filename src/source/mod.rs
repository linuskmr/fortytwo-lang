//! Abstractions of source code.
//!
//! Every char in source code belongs to a [Source] (e.g. a file) and has a [Position] in this file.
//! To make it more ergonomically to work with positions, the [PositionContainer] wraps an element
//! with its [Position].

mod position;
mod position_container;
mod position_range;
mod source_position;

use std::fmt;
use std::sync::Arc;
pub use position::Position;
pub use position_container::PositionContainer;
pub use position_range::PositionRange;
pub use source_position::SourcePositionRange;


/// Contains the source code of a file.
///
/// Mostly used as `Arc<Source>`, since this is cheaper to clone.
#[derive(Eq, PartialEq, Hash)]
pub struct Source {
	/// Filename.
	pub name: String,
	/// Content as lines of chars.
	pub text: Vec<Vec<char>>,
}

impl Source {

	/// Creates a [Source] from a filename and the content.
	///
	/// # Example
	///
	/// ```
	/// use fortytwolang::source::Source;
	///
	/// let source = Source::new("file.name".to_owned(), "ab\nc".to_owned());
	/// assert_eq!(source.name, "file.name");
	/// assert_eq!(source.text, vec![vec!['a', 'b'], vec!['c']]);
	/// ```
	pub fn new(name: String, text: String) -> Self {
		Self {
			name,
			text: text.lines().map(|line| line.chars().collect()).collect(),
		}
	}

	/// Creates an iterator over the [`Symbol`]s of the source code.
	///
	/// # Example
	///
	/// ```
	/// use std::sync::Arc;
	/// use fortytwolang::source::{Position, PositionContainer, PositionRange, Source, SourcePositionRange};
	///
	/// let source = Arc::new(Source::new(
	/// 	"file.name".to_owned(),
	/// 	"text...".to_owned(),
	/// ));
	/// let mut iter = Arc::clone(&source).iter();
	/// let expected = Some(PositionContainer::new('t', SourcePositionRange {
	/// 	source: Arc::clone(&source),
	/// 	position: PositionRange {
	/// 		start: Position {
	/// 			line: 0,
	/// 			column: 0,
	/// 		},
	/// 		end: Position {
	/// 			line: 0,
	/// 			column: 0,
	/// 		}
	/// 	}
	/// }));
	/// assert_eq!(iter.next(), expected);
	/// ```
	pub fn iter(self: Arc<Self>) -> impl Iterator<Item=Symbol> {
		SourceIter {
			source: self,
			position: Position::default(),
		}
	}
}

impl fmt::Debug for Source {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Source")
			.field("name", &self.name)
			.finish()
	}
}


/// A symbol is the name for [`SourcePositionRange<char>`], i.e. a char with its position in the source code.
pub type Symbol = PositionContainer<char>;


/// Iterator over the chars of a source code.
struct SourceIter {
	/// Source code
	source: Arc<Source>,
	/// Current index in the source code.
	position: Position,
}

impl Iterator for SourceIter {
	type Item = Symbol;

	fn next(&mut self) -> Option<Self::Item> {
		let line = self.source.text.get(self.position.line)?;

		let char_ = line.get(self.position.column);
		let char_ = match char_ {
			Some(&char_ ) => char_,
			// Since self.source.text doesn't contain newlines, we have to insert them back
			None => '\n',
		};

		let item = PositionContainer::new(
			char_,
			SourcePositionRange {
				source: Arc::clone(&self.source),
				position: PositionRange {
					start: self.position,
					end: self.position,
				}
			},
		);

		if char_ == '\n' {
			self.position.line += 1;
			self.position.column = 0;
		} else {
			self.position.column += 1;
		}

		Some(item)
	}
}


#[cfg(test)]
mod tests {
	use std::sync::Arc;
	use super::*;


	#[test]
	fn test_source_iter() {
		let source = Arc::new(Source::new(
			"file.name".to_owned(),
			 "ab\nc".to_owned(),
		));

		let mut iter = Arc::clone(&source).iter();

		assert_eq!(iter.next(), Some(PositionContainer::new('a', SourcePositionRange {
			source: Arc::clone(&source),
			position: PositionRange {
				start: Position {
					line: 0,
					column: 0,
				},
				end: Position {
					line: 0,
					column: 0,
				},
			},
		})));
		assert_eq!(iter.next(), Some(PositionContainer::new('b', SourcePositionRange {
			source: Arc::clone(&source),
			position: PositionRange {
				start: Position {
					line: 0,
					column: 1,
				},
				end: Position {
					line: 0,
					column: 1,
				},
			},
		})));
		assert_eq!(iter.next(), Some(PositionContainer::new('\n', SourcePositionRange {
			source: Arc::clone(&source),
			position: PositionRange {
				start: Position {
					line: 0,
					column: 2,
				},
				end: Position {
					line: 0,
					column: 2,
				},
			},
		})));
		assert_eq!(iter.next(), Some(PositionContainer::new('c', SourcePositionRange {
			source: Arc::clone(&source),
			position: PositionRange {
				start: Position {
					line: 1,
					column: 0,
				},
				end: Position {
					line: 1,
					column: 0,
				},
			},
		})));
		assert_eq!(iter.next(), Some(PositionContainer::new('\n', SourcePositionRange {
			source: Arc::clone(&source),
			position: PositionRange {
				start: Position {
					line: 1,
					column: 1,
				},
				end: Position {
					line: 1,
					column: 1,
				},
			},
		})));
		assert_eq!(iter.next(), None);
	}
}
