use crate::position_container::{PositionRange, PositionRangeContainer};
use crate::position_container::PositionContainer;
use crate::position_reader::{IndexReader, Symbol};
use crate::token::{Token, TokenType};
use crate::error::{ParsingError, ParsingErrorKind};

/// A lexer consuming the sourcecode line-by-line and returning the parsed tokens.
pub struct Lexer<R: Iterator<Item=String>> {
    /// The source to read from.
    reader: IndexReader<R>,
    /// The current symbol what is being processed.
    current_symbol: Option<Symbol>,
}

impl<R: Iterator<Item=String>> Lexer<R> {
    /// Creates a new Lexer with the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader: IndexReader::new(reader),
            current_symbol: None,
        }
    }

    /// Loads the next symbol from [self.reader], saves it into [self.current_symbol] and returns it.
    fn get_next_symbol(&mut self) -> Option<&Symbol> {
        self.current_symbol = self.reader.next();
        self.current_symbol.as_ref()
    }

    /// Discards all whitespace and newlines until a non-whitespace symbol is found.
    pub(crate) fn read_until_not_whitespace(&mut self) -> Option<&Symbol> {
        let mut reader_drained = false;
        loop {
            match &self.current_symbol {
                Some(Symbol { data: c, .. }) if c.is_whitespace() || *c == '\n' => (),
                Some(_) => break,
                // Here you don't know if a symbol has never been read, or if the reader is already drained. If
                // the reader does not supply a symbol in the next loop run, it is drained.
                None if reader_drained => break,
                None => reader_drained = true,
            };
            self.get_next_symbol();
        }
        self.current_symbol.as_ref()
    }

    fn tokenize_next_item(&mut self) -> Option<Result<Token, ParsingError>> {
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
                None => return Some(Err(ParsingError::from_symbol(
                    &symbol,
                    ParsingErrorKind::UnknownSymbol,
                    format!("Unknown token `{}`", symbol.data),
                ))),
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

impl<R: Iterator<Item=String>> Iterator for Lexer<R> {
    type Item = Result<Token, ParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize_next_item()
    }
}