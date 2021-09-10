use crate::position_container::{Position, PositionContainer};
use std::iter::FusedIterator;

/// A char combined with position information, i.e. the line and column number where that char was read.
pub type Symbol = PositionContainer<char>;

/// The first line number to start with.
const START_LINE_NR: usize = 1;
/// The first column number to start with.
const START_COLUMN_NR: usize = 1;

/// PositionReader reads chars from the [source](PositionReader::source) and returns them combined with information in which line and column
/// they were read in.
///
/// ```
/// # use fortytwo_lang::position_reader::{PositionReader, Symbol};
/// # use fortytwo_lang::position_container::Position;
/// let source = "Hello\nWorld";
/// let mut position_reader = PositionReader::new(source.chars());
/// let expected = [
///     Symbol { data: 'H', position: Position { line: 1, column: 1 } },
///     Symbol { data: 'e', position: Position { line: 1, column: 2 } },
///     Symbol { data: 'l', position: Position { line: 1, column: 3 } },
///     Symbol { data: 'l', position: Position { line: 1, column: 4 } },
///     Symbol { data: 'o', position: Position { line: 1, column: 5 } },
///     Symbol { data: '\n', position: Position { line: 1, column: 6 } },
///     Symbol { data: 'W', position: Position { line: 2, column: 1 } },
///     Symbol { data: 'o', position: Position { line: 2, column: 2 } },
///     Symbol { data: 'r', position: Position { line: 2, column: 3 } },
///     Symbol { data: 'l', position: Position { line: 2, column: 4 } },
///     Symbol { data: 'd', position: Position { line: 2, column: 5 } }
/// ];
/// assert!(position_reader.eq(expected));
/// ```
pub struct PositionReader<S: Iterator<Item=char>> {
    /// The source to read from.
    source: S,
    /// Bookkeeping of the current line number.
    line: usize,
    /// Bookkeeping of the current line number.
    column: usize,
}

impl<S: Iterator<Item=char>> PositionReader<S> {
    /// Creates a new [PositionReader] with the given source.
    pub fn new(source: S) -> Self {
        Self {
            source,
            line: START_LINE_NR,
            column: START_COLUMN_NR
        }
    }
}

impl<S: Iterator<Item=char>> Iterator for PositionReader<S> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        // Read char and add position information
        let symbol = self.source.next()
            .map(|read_char| Symbol {
                data: read_char,
                position: Position { line: self.line, column: self.column }
            });

        // Increment column and/or line counter
        match &symbol {
            Some(symbol) if symbol.data == '\n' => {
                // Newline: Increment line, reset column
                self.line += 1;
                self.column = START_COLUMN_NR;
            },
            Some(_) => {
                // Normal char: Increment column
                self.column += 1;
            },
            None => ()
        }
        symbol
    }
}

impl<S: FusedIterator<Item=char>> FusedIterator for PositionReader<S> {}