use std::ops::RangeInclusive;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PositionContainer<T> {
    /// The data of this container.
    pub data: T,
    pub position: Position,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PositionRangeContainer<T> {
    /// The data of this container.
    pub data: T,
    pub position: PositionRange,
}

impl<T> PositionRangeContainer<T> {
    #[allow(unused)]
    pub(crate) fn new(data: T, position: PositionRange) -> Self {
        Self {
            data,
            position
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PositionRange {
    pub line: usize,
    pub column: RangeInclusive<usize>,
}

impl PositionRange {
    pub(crate) fn from_start(start: Position) -> Self {
        Self {
            line: start.line,
            column: start.column..=start.column
        }
    }

    pub(crate) fn set_end(&mut self, end: &Position) {
        self.column = *self.column.start()..=end.column
    }
}

impl From<&Position> for PositionRange {
    fn from(position: &Position) -> Self {
        Self {
            line: position.line,
            column: position.column..=position.column
        }
    }
}

impl fmt::Display for PositionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}..{}", self.line, self.column.start(), self.column.end())
    }
}