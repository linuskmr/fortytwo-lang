use std::io;

use crate::position_container::{Position, PositionContainer};

pub(crate) type Symbol = PositionContainer<SymbolType>;

#[derive(Debug, Clone)]
pub(crate) enum SymbolType {
    Character(char),
    Whitespace,
    Newline,
}

/// The IndexReader reads Symbol's and returns in which row and column they were read.
pub(crate) struct IndexReader<R: io::Read> {
    /// The source to read from.
    reader: R,
    /// This field does not affect the underlying read operation on the reader. It is used
    /// only for the returned symbols.
    next_read_position: Position,
    /// The last symbol this reader has read or None if no symbol has been read yet.
    current_symbol: Option<Symbol>,
}

impl<R: io::Read> IndexReader<R> {
    /// Creates a new CharReader with the given reader.
    pub(crate) fn new(reader: R) -> Self {
        Self {
            reader,
            next_read_position: Position { line: 0, column: 0 },
            current_symbol: None,
        }
    }

    /// Returns the current symbol, so the last read symbol. If the underlying reader returns
    /// None or no symbol has yet been read, this method returns None.
    pub(crate) fn current(&self) -> Option<Symbol> {
        self.current_symbol.clone()
    }
}

impl<R: io::Read> Iterator for IndexReader<R> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        // Indices in which the current character was read
        let position = self.next_read_position.clone();

        // Read next char
        let mut buffer = [0u8; 1];
        let symbol_type = match self.reader.read(&mut buffer).unwrap() {
            0 => None, // self.reader is empty -> EOF
            _ if (buffer[0] as char).is_whitespace() => {
                // Whitespace
                self.next_read_position.column += 1;
                Some(SymbolType::Whitespace)
            }
            _ if (buffer[0] as char) == '\n' => {
                // Newline
                self.next_read_position.line += 1;
                self.next_read_position.column = 0;
                Some(SymbolType::Newline)
            }
            _ => {
                // Normal character
                self.next_read_position.column += 1;
                Some(SymbolType::Character(buffer[0] as char))
            }
        };

        self.current_symbol = match symbol_type {
            None => None, // self.reader is empty
            Some(symbol_type) => Some(Symbol {
                data: symbol_type,
                position,
            }),
        };
        self.current()
    }
}