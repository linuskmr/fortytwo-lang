use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeInclusive;
use std::convert::TryInto;

/// The first line number to start with.
pub(crate) const START_LINE_NR: usize = 1;
/// The first column number to start with.
pub(crate) const START_COLUMN_NR: usize = 1;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionContainer<T> {
    /// The data of this container.
    pub data: T,
    pub position: Position,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionRangeContainer<T> {
    /// The data of this container.
    pub data: T,
    pub position: PositionRange,
}

impl<T> PositionRangeContainer<T> {
    pub(crate) fn new(data: T, position: PositionRange) -> Self {
        Self { data, position }
    }
}

impl<T: Debug> Display for PositionRangeContainer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} in line={} column={}..{}",
            self.data,
            self.position.line,
            self.position.column.start(),
            self.position.column.end()
        )
    }
}

impl<T> PositionRangeContainer<T> {
    pub(crate) fn try_from2<E: TryInto<T>>(value: PositionRangeContainer<E>) -> Result<Self, E::Error> {
        // This TryFrom implementation enable to convert a &PositionRangeContainer with type E to a
        // PositionRangeContainer with type T, if TryFrom is implemented for converting an &E to T. Because copying the
        // position from the other PositionRangeContainer can not fail, this function will only throws errors produced
        // by TryFrom(&E -> T).
        Ok(Self {
            data: value.data.try_into()?,
            position: value.position.clone()
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            line: START_LINE_NR,
            column: START_COLUMN_NR
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionRange {
    pub line: usize,
    pub column: RangeInclusive<usize>,
}

impl PositionRange {
    pub(crate) fn from_start(start: Position) -> Self {
        Self {
            line: start.line,
            column: start.column..=start.column,
        }
    }

    pub(crate) fn set_end(&mut self, end: &Position) {
        self.column = *self.column.start()..=end.column
    }
}

impl Default for PositionRange {
    fn default() -> Self {
        Self {
            line: START_LINE_NR,
            column: START_COLUMN_NR..=START_COLUMN_NR
        }
    }
}

impl From<&Position> for PositionRange {
    fn from(position: &Position) -> Self {
        Self {
            line: position.line,
            column: position.column..=position.column,
        }
    }
}
