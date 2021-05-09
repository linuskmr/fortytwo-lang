use std::io;
use std::iter::Enumerate;
use std::str::Chars;

use crate::position_container::{Position, PositionContainer};

pub(crate) type Symbol = PositionContainer<char>;

/// The IndexReader reads Symbol's and returns in which row and column they were read.
pub(crate) struct IndexReader<'a, R: Iterator<Item=String>> {
    /// The source to read from.
    line_reader: R,
    /// This field does not affect the underlying read operation on the reader. It is used
    /// only for the returned symbols.
    line_index: usize,
    raw_line: String,
    chars_in_line: Option<Enumerate<Chars<'a>>>,
}

impl<'a, R: Iterator<Item=String>> IndexReader<'a, R> {
    /// Creates a new CharReader with the given reader.
    pub(crate) fn new(reader: R) -> Self {
        Self {
            line_reader: reader,
            line_index: 0,
            raw_line: String::from(""),
            chars_in_line: None,
        }
    }

    fn next_line(&mut self) -> bool {
        match self.line_reader.next() {
            Some(line) => {
                self.line_index += 1;
                self.raw_line = line;
                self.chars_in_line = Some(self.raw_line.chars().enumerate());
                true
            }
            None => {
                self.raw_line = String::from("");
                self.chars_in_line = None;
                false
            }
        }
    }

    fn get_next_char(&mut self) -> Option<Symbol> {
        match self.chars_in_line.next() {
            Some(c) => c,
            None => {
                // Current line is empty, so get the next line
                while self.next_line() {
                    if let Some((column, _char)) = self.chars_in_line.next() {
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

impl<'a, R: Iterator<Item=String>> Iterator for IndexReader<'a, R> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_char()
    }
}