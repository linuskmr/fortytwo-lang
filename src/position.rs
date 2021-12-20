use std::fmt::Debug;
use miette;

/*#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionContainer<T> {
    /// The data of this container.
    pub data: T,
    pub position: miette::SourceSpan,
}
*/

pub type PositionContainer<T> = (PositionRange, T);

/// Span within source code.
/// Inspired by miette https://crates.io/crates/miette/3.2.0
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PositionRange {
    /// The start of the span, i.e. the the start index.
    pub(crate) start: usize,
    /// The length of the position, i.e. how many chars after start belong to this position.
    pub(crate) length: usize,
}

impl From<(usize, usize)> for PositionRange {
    fn from((start, length): (usize, usize)) -> Self {
        Self { start, length }
    }
}

impl From<PositionRange> for miette::SourceSpan {
    fn from(position: PositionRange) -> Self {
        PositionRange::new(position.start.into(), position.length.into())
    }
}