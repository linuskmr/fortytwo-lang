use std::iter::Enumerate;

use crate::position_container::{Position, PositionContainer};

pub(crate) type Symbol = PositionContainer<char>;

/// The IndexReader reads [Symbols](Symbol) and returns in which line and column they were read.
pub(crate) struct IndexReader<R: Iterator<Item=String>> {
    /// The source to read from.
    line_reader: R,
    /// Bookkeeping of the current line number.
    line_nr: usize,
    /// An iterator over the chars of the current line.
    chars_in_line: Enumerate<std::vec::IntoIter<char>>,
}

impl<R: Iterator<Item=String>> IndexReader<R> {
    /// Creates a new [IndexReader] with the given reader.
    pub(crate) fn new(reader: R) -> Self {
        Self {
            line_reader: reader,
            line_nr: 0,
            chars_in_line: vec![].into_iter().enumerate(),
        }
    }

    /// Loads the next line. Returns true if there is a next line, otherwise returns false.
    fn next_line(&mut self) -> bool {
        match self.line_reader.next() {
            Some(line) => {
                self.line_nr += 1;
                self.chars_in_line = line.chars().collect::<Vec<char>>().into_iter().enumerate();
                true
            }
            None => {
                self.chars_in_line = vec![].into_iter().enumerate();
                false
            }
        }
    }
}

impl<R: Iterator<Item=String>> Iterator for IndexReader<R> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars_in_line.next() {
            Some((column, c)) => Some(Symbol {
                data: c,
                position: Position { line: self.line_nr, column },
            }),
            None => {
                // Current line is empty, so get the next line
                while self.next_line() {
                    if let Some((column, c)) = self.chars_in_line.next() {
                        // Found a char
                        return Some(Symbol {
                            data: c,
                            position: Position { line: self.line_nr, column },
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