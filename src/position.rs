use std::fmt::{Debug, Formatter};
use std::ops::RangeInclusive;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionContainer<T> {
    /// The data of this container.
    pub value: T,
    pub position: Position,
}


/// Position in the source code ranging from start to end (both inclusive).
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Position {
    position: RangeInclusive<usize>,
}

impl Position {
    pub fn from_start_end(start: usize, end: usize) -> Self {
        Self {
            position: start..=end
        }
    }

    pub fn from_start_len(start: usize, len: usize) -> Self {
        Self {
            position: start..=(start+len)
        }
    }

    /// Index in the source code where the position begins (inclusive).
    pub fn start(&self) -> usize {
        *self.position.start()
    }

    /// Index in the source code where the position ends (inclusive).
    pub fn end(&self) -> usize {
        *self.position.end()
    }

    /// Length of the position.
    pub fn len(&self) -> usize {
        self.end() - self.start()
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.position.fmt(f)
    }
}

impl From<Position> for miette::SourceSpan {
    fn from(position: Position) -> Self {
        miette::SourceSpan::new(position.start().into(), position.len().into())
    }
}