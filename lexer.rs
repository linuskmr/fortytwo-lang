use crate::position_container::{PositionRange, PositionRangeContainer};
use crate::position_reader::{PositionReader, Symbol};
use crate::token::{Token, TokenType};
use crate::error::{ParsingError, ParsingErrorKind};
use std::iter::Peekable;


/// A lexer is an iterator that consumes the FTL sourcecode char-by-char and returns the parsed [Token]s.
pub struct Lexer<S: Iterator<Item=char>> {
    /// The source to read from.
    symbols: Peekable<PositionReader<S>>,
    last_comment: Option<String>
}

impl<S: Iterator<Item=char>> Lexer<S> {
    /// Creates a new Lexer from the given symbol reader.
    pub fn new(symbols: S) -> Self {
        Self {
            symbols: PositionReader::new(symbols).peekable(),
            last_comment: None
        }
    }

    /// Checks if [Lexer.symbols] stands on a non-whitespace char.
    fn on_non_whitespace(&mut self) -> bool {
        match self.symbols.peek() {
            Some(symbol) if !symbol.data.is_whitespace() => true,
            None => true,
            _ => false,
        }
    }

    /// Skips all chars of [Lexer.symbols] until the first non-whitespace is found. This function does always
    /// advances [Lexer.symbols], even if the current symbol is a non-whitespace char.
    fn skip_whitespaces(&mut self) {
        loop {
            self.symbols.next();
            match &self.symbols.peek() {
                Some(Symbol { data: c, .. }) if c.is_whitespace() => (),
                Some(_) | None  => break,
            };
        }
    }

    /// Goes to the first non-whitespace char in [Lexer.symbols]. If [Lexer.symbols] already stands on a
    /// non-whitespace char, this function does return immediate. Else [Lexer.skip_whitespaces()] is called, which
    /// skips all chars until the first non-whitespace.
    ///
    /// ```
    /// use ftllib::lexer::Lexer;
    /// let lexer = Lexer::new();
    /// ```
    pub(crate) fn goto_non_whitespace(&mut self) {
        // Return early if `self.symbols` already stands on a non-whitespace char
        if self.on_non_whitespace() {
            return
        }
        // Search first non-whitespace char
        self.skip_whitespaces();
    }

    /// Tokenizes the next symbol from [Lexer::symbols]. Returns `None` if the symbol can be ignored (e.g. comment or
    /// carriage return). That means that this function does not always return all `Some`'s first and than always None,
    /// like an iterator does. To check if there are no more symbols to tokenize, check if [self.symbols.peek()] is
    /// None.
    /// Furthermore this function assumes that [Lexer::symbols] does not yield a whitespace character, so you have to
    /// check that before calling this function. Otherwise this function will return an `Err`, because of an unknown
    /// Symbol.
    fn tokenize_next_item(&mut self) -> Option<Result<Token, ParsingError>> {
        match self.symbols.peek()? {
            Symbol {data, .. } if data.is_alphabetic() => {
                // String
                Some(parse_string(self.read_string()))
            },
            Symbol {data, ..} if data.is_numeric() => {
                // Number
                Some(parse_number(self.read_number()))
            },
            Symbol {data, ..} if is_comment(*data) => {
                // Comment
                self.skip_comment_line();
                None
            },
            Symbol {data, ..} if *data == '\r' => {
                // Ignore carriage return
                self.symbols.next();
                None
            }
            Symbol {data, ..} if *data == '\n' => {
                // Newline
                todo!("Handle newline")
            }
            Symbol {data, ..} if is_special_char(*data) => {
                // Special character
                todo!("Special character")
            }
            Symbol { data, position } => {
                // Unknown symbol
                Some(Err(ParsingError {
                    kind: ParsingErrorKind::UnknownSymbol,
                    msg: format!("Unknown symbol `{}`", data),
                    position: position.into()
                }))
            }
        }
    }

    /// Reads a string from [Lexer::symbols].
    fn read_string(&mut self) -> PositionRangeContainer<String> {
        let first_symbol = self.symbols.next().expect("read_string called on empty `self.symbols`");
        let mut position = PositionRange::from_start(first_symbol.position.clone());
        let mut identifier = String::from(first_symbol.data);

        loop {
            // Add chars to the identifier as long as there are chars or numeric
            match self.symbols.peek() {
                Some(Symbol { data: symbol, position: symbol_position, .. }) if symbol.is_alphanumeric() => {
                    position.set_end(symbol_position);
                    identifier.push(*symbol);
                }
                _ => break,
            }
            self.symbols.next();
        }
        PositionRangeContainer { data: identifier, position }
    }

    /// Reads a number from [Lexer::symbols].
    fn read_number(&mut self) -> PositionRangeContainer<String> {
        let first_symbol = self.symbols.next().expect("read_number called on empty `self.symbols`");
        assert!(first_symbol.data.is_numeric());
        let mut position = PositionRange::from_start(first_symbol.position);
        let mut number = String::from(first_symbol.data);
        loop {
            // Add chars to the number as long as there are numeric or a dot
            match self.symbols.peek() {
                Some(Symbol { data: c, position: symbol_position }) if c.is_numeric() => {
                    position.set_end(symbol_position);
                    number.push(*c);
                }
                Some(Symbol { data: c, position: symbol_position }) if *c == '.' => {
                    // Number is a float
                    position.set_end(symbol_position);
                    number.push(*c);
                }
                _ => break,
            }
            self.symbols.next();
        }
        PositionRangeContainer { data: number, position }
    }

    /// Skips a comment line in [self.symbols] and stores it in [self.last_comment].
    fn skip_comment_line(&mut self) {
        loop {
            match self.symbols.peek() {
                Some(Symbol { data: '\n', .. }) | None => break,
                _ => (), // Skip all other chars. This includes \r.
            }
            self.symbols.next();
        }
    }
}

/// Parses a string to a keyword or to an identifier.
///
/// # Panics
///
/// Panics if the string is empty.
fn parse_string(string: PositionRangeContainer<String>) -> Result<Token, ParsingError> {
    assert!(!string.data.is_empty(), "Identifier must not be empty");
    // TODO: Extract match statement to HashMap.
    Ok(match string.data.as_str() {
        "def" => Token { data: TokenType::Def, position: string.position },
        "extern" => Token { data: TokenType::Extern, position: string.position },
        _ => Token { data: TokenType::Identifier(string.data), position: string.position },
    })
}

/// Parses a number to a [TokenType::Number}].
fn parse_number(number: PositionRangeContainer<String>) -> Result<Token, ParsingError> {
    // TODO: Add parsing for other number types.
    let parsed_number: f64 = match number.data.parse() {
        Ok(num) => num,
        Err(e) => return Err(ParsingError{
            kind: ParsingErrorKind::UnknownSymbol,
            msg: e.to_string(),
            position: number.position
        })
    };
    Ok(Token { data: TokenType::Number(parsed_number), position: number.position })
}

/// Checks if `symbol` starts a comment line.
///
/// ```
/// assert!(is_comment('#'));
/// assert!(!is_comment('1'));
/// ```
fn is_comment(symbol: char) -> bool {
    symbol == '#'
}

/// Checks if `symbol` is a special character like `+`, `-`, `=`, `*`.
fn is_special_char(symbol: char) -> bool {
    symbol == '+' || symbol == '-' || symbol == '=' || symbol == '*'
}

impl<S: Iterator<Item=char>> Iterator for Lexer<S> {
    type Item = Result<Token, ParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Make self.symbols not return a whitespace, which is assumed by `self.tokenize_next_item()`
            self.goto_non_whitespace();
            // If self.symbols is drained, we will return here
            self.symbols.peek()?;
            // Tokenize returned a token? Then return it, otherwise try again
            if let Some(token) = self.tokenize_next_item() {
                return Some(token)
            }
        }
    }
}

