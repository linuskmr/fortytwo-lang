use std::io;

use crate::position_container::PositionContainer;
pub use crate::position_container::{PositionRange, PositionRangeContainer};
use crate::position_reader::{IndexReader, Symbol, SymbolType};
pub use crate::token::{Token, TokenType, SpecialCharacter};

pub mod token;
mod position_reader;
mod position_container;

pub struct Lexer<R: io::Read> {
    /// The source to read from.
    reader: IndexReader<R>,
}

impl<R: io::Read> Lexer<R> {
    /// Creates a new Lexer with the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader: IndexReader::new(reader),
        }
    }

    /// Discards all whitespace symbols until a non-whitespace symbol is found.
    pub(crate) fn read_until_not_whitespace(&mut self) -> Option<Symbol> {
        let mut reader_drained = false;
        loop {
            match self.reader.current() {
                symbol @ Some(Symbol { data: SymbolType::Character(_), .. }) => {
                    return symbol;
                }
                None if reader_drained => return None,
                None => reader_drained = true,
                // Ignore whitespace and newline
                _ => (),
            };
            self.reader.next();
        }
    }

    fn tokenize_next_item(&mut self) -> Option<Token> {
        let symbol = self.read_until_not_whitespace()?;
        let current_char = match symbol {
            Symbol { data: SymbolType::Character(c), .. } => c,
            _ => panic!("read_until_not_whitespace returned an invalid char"),
        };
        if current_char.is_alphabetic() { // Identifier
            let identifier = self.read_identifier(PositionContainer {
                data: current_char,
                position: symbol.position,
            });
            return Some(parse_identifier(identifier));
        } else if current_char.is_numeric() { // Number
            let number_string = self.read_number_string(PositionContainer {
                data: current_char,
                position: symbol.position,
            });
            return Some(parse_float(number_string));
        } else if current_char == '#' { // Comment line
            self.ignore_comment();
            return self.tokenize_next_item();
        } else { // Other
            let other = Token {
                data: TokenType::Other(SpecialCharacter(current_char)),
                position: PositionRange::from_start(symbol.position),
            };
            self.reader.next(); // Consume current char
            return Some(other);
        }
    }

    /// Reads an identifier to a string.
    fn read_identifier(&mut self, current_char: PositionContainer<char>) -> PositionRangeContainer<String> {
        let mut position = PositionRange::from_start(current_char.position);
        let mut identifier = String::from(current_char.data);
        loop {
            // Add chars to the identifier as long as there are chars
            match self.reader.next() {
                Some(Symbol { data: SymbolType::Character(c), position: symbol_position }) if c.is_alphanumeric() => {
                    position.update_end(symbol_position);
                    identifier.push(c);
                }
                _ => break,
            }
        }
        PositionRangeContainer {
            data: identifier,
            position,
        }
    }

    /// Reads a number to a string.
    fn read_number_string(&mut self, current_char: PositionContainer<char>) -> PositionRangeContainer<String> {
        let mut position = PositionRange::from_start(current_char.position);
        let mut number = String::from(current_char.data);
        loop {
            // Add chars to the number as long as there are numeric
            match self.reader.next() {
                Some(Symbol { data: SymbolType::Character(c), position: symbol_position }) if c.is_numeric() => {
                    position.update_end(symbol_position);
                    number.push(c);
                }
                Some(Symbol { data: SymbolType::Character(c), position: symbol_position }) if c == '.' => {
                    position.update_end(symbol_position);
                    number.push(c);
                }
                _ => break,
            }
        }
        PositionRangeContainer::new(number, position)
    }

    /// Skips a comment line.
    fn ignore_comment(&mut self) {
        loop {
            match self.reader.next() {
                Some(Symbol { data: SymbolType::Newline, .. }) | None => break,
                _ => (),
            }
        }
    }
}

/// Parses an identifier to a keyword or to an identifier.
///
/// # Panics
///
/// Panics if the identifier string is empty.
fn parse_identifier(identifier: PositionRangeContainer<String>) -> Token {
    assert!(identifier.data.len() >= 1, "Identifier must not be empty");
    match identifier.data.as_str() {
        "def" => Token { data: TokenType::Def, position: identifier.position },
        "extern" => Token { data: TokenType::Extern, position: identifier.position },
        _ => Token { data: TokenType::Identifier(identifier.data), position: identifier.position },
    }
}

/// Parses a float as string to a [TokenType::Number}].
///
/// # Panics
///
/// Panics if float_string is not parsable.
fn parse_float(float_string: PositionRangeContainer<String>) -> Token {
    let value = float_string.data.parse().unwrap();
    Token { data: TokenType::Number(value), position: float_string.position }
}

impl<R: io::Read> Iterator for Lexer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize_next_item()
    }
}