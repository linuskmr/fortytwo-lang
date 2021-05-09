use std::iter::Enumerate;
use std::str::Chars;

use crate::position_container::{Position, PositionContainer};

pub(crate) type Symbol = PositionContainer<char>;

/// The IndexReader reads [Symbols](Symbol) and returns in which line and column they were read.
pub(crate) struct IndexReader<R: Iterator<Item=String>> {
    /// The source to read from.
    line_reader: R,
    /// Bookkeeping of the current line number.
    line_index: usize,
    /// An iterator over the chars of the current [raw_line].
    chars_in_line: Option<Enumerate<std::vec::IntoIter<char>>>,
}

impl<R: Iterator<Item=String>> IndexReader<R> {
    /// Creates a new [IndexReader] with the given reader.
    pub(crate) fn new(reader: R) -> Self {
        Self {
            line_reader: reader,
            line_index: 0,
            chars_in_line: None,
        }
    }

    /// Loads the next line. Returns true if there is a next line, otherwise returns false.
    fn next_line(&mut self) -> bool {
        match self.line_reader.next() {
            Some(line) => {
                self.line_index += 1;
                self.chars_in_line = Some(line.chars().collect::<Vec<char>>().into_iter().enumerate());
                true
            }
            None => {
                self.chars_in_line = None;
                false
            }
        }
    }

    /// Returns the next symbol, or none if [reader] is drained.
    fn get_next_symbol(&mut self) -> Option<Symbol> {
        match self.chars_in_line.as_mut().and_then(|it| it.next()) {
            Some((column, _char)) => Some(Symbol{
                data: _char,
                position: Position { line: self.line_index, column }
            }),
            None => {
                // Current line is empty, so get the next line
                while self.next_line() {
                    let chars_in_line = match &mut self.chars_in_line {
                        Some(chars_in_line) => chars_in_line,
                        None => return None,
                    };
                    if let Some((column, _char)) = chars_in_line.next() {
                        // Found a char
                        return Some(Symbol {
                            data: _char,
                            position: Position { line: self.line_index, column },
                        });
                    }
                    // This line was empty. Try next line
                }
                // line_reader drained
                None
            }
        }
    }
}

impl<R: Iterator<Item=String>> Iterator for IndexReader<R> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_symbol()
    }
}