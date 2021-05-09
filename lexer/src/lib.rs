use std::io;

pub use crate::error::UnknownSymbolError;
pub use crate::position_container::{PositionRange, PositionRangeContainer};
use crate::position_container::PositionContainer;
use crate::position_reader::{IndexReader, Symbol};
pub use crate::token::{Token, TokenType};

pub mod token;
mod position_reader;
mod position_container;
mod error;

pub struct Lexer<'a, R: Iterator<Item=String>> {
    /// The source to read from.
    reader: IndexReader<'a, R>,
    current_symbol: Option<Symbol>,
}

impl<'a, R: Iterator<Item=String>> Lexer<'a, R> {
    /// Creates a new Lexer with the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader: IndexReader::new(reader),
            current_symbol: None,
        }
    }

    fn get_next_symbol(&mut self) -> Option<&Symbol> {
        self.current_symbol = self.reader.next();
        self.current_symbol.as_ref()
    }

    /// Discards all whitespace symbols until a non-whitespace symbol is found.
    pub(crate) fn read_until_not_whitespace(&mut self) -> Option<&Symbol> {
        let mut reader_drained = false;
        loop {
            match &self.current_symbol {
                Some(Symbol { data: c, .. }) if c.is_whitespace() || *c == '\n' => (),
                Some(symbol) => return Some(symbol),
            };
            self.reader.next();
        }
    }

    fn tokenize_next_item(&mut self) -> Option<Result<Token, UnknownSymbolError>> {
        let symbol = self.read_until_not_whitespace().cloned()?;
        if symbol.data.is_alphabetic() { // Identifier
            let identifier = self.read_identifier(PositionContainer {
                data: symbol.data,
                position: symbol.position,
            });
            return Some(Ok(parse_identifier(identifier)));
        } else if symbol.data.is_numeric() { // Number
            let number_string = self.read_number_string(PositionContainer {
                data: symbol.data,
                position: symbol.position,
            });
            return Some(Ok(parse_float(number_string)));
        } else if symbol.data == '#' { // Comment line
            self.ignore_comment();
            return self.tokenize_next_item();
        } else { // Other
            let token = Token::from_symbol(symbol.clone());
            let token = match token {
                None => return Some(Err(UnknownSymbolError::from_symbol(&symbol))),
                Some(tok) => tok
            };
            self.reader.next(); // Consume current char
            return Some(Ok(token));
        }
    }

    /// Reads an identifier to a string.
    fn read_identifier(&mut self, current_char: PositionContainer<char>) -> PositionRangeContainer<String> {
        let mut position = PositionRange::from_start(current_char.position);
        let mut identifier = String::from(current_char.data);
        loop {
            // Add chars to the identifier as long as there are chars
            match self.reader.next() {
                Some(Symbol { data: c, position: symbol_position, .. }) if c.is_alphanumeric() => {
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
                Some(Symbol { data: c, position: symbol_position }) if c.is_numeric() => {
                    position.update_end(symbol_position);
                    number.push(c);
                }
                Some(Symbol { data: c, position: symbol_position }) if c == '.' => {
                    position.update_end(symbol_position);
                    number.push(c);
                }
                _ => break,
            }
        }
        PositionRangeContainer { data: number, position }
    }

    /// Skips a comment line.
    fn ignore_comment(&mut self) {
        loop {
            match self.reader.next() {
                Some(Symbol { data: '\n', .. }) | None => break,
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

impl<'a, R: Iterator<Item=String>> Iterator for Lexer<'a, R> {
    type Item = Result<Token, UnknownSymbolError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize_next_item()
    }
}