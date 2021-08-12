use crate::position_container::{Position, PositionContainer};

/// A char combined with position information, i.e. the line and column number where that char was read.
pub(crate) type Symbol = PositionContainer<char>;

/// The first line number to start with.
const START_LINE_NR: usize = 1;
/// The first column number to start with.
const START_COLUMN_NR: usize = 1;

/// PositionReader reads chars from the source S and returns them combined with information in which line and column
/// they were read in.
///
/// ```
/// use ftllib::position_reader::PositionReader;
/// let source = "Hello\nWorld";
/// let mut pr = PositionReader::new(source.chars());
/// assert_eq!(pr.next(), Some(Symbol {data: 'H', position}));
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
        Self { source, line:START_LINE_NR, column: START_COLUMN_NR }
    }
}

impl<R: Iterator<Item=char>> Iterator for PositionReader<R> {
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