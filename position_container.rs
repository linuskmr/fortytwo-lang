extern crate serde;
use serde::Serialize;
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct PositionContainer<T> {
    /// The data of this container.
    pub(crate) data: T,
    pub(crate) position: Position,
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug)]
pub(crate) struct Position {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

#[derive(Clone, Debug, Serialize)]
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

    pub(crate) fn update_end(&mut self, end: Position) {
        self.column = *self.column.start()..=end.column
    }
}