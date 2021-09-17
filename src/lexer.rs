use std::borrow::Borrow;
use std::iter::Peekable;

use crate::error::{FTLError, FTLErrorKind, ParseResult};
use crate::position_container::{PositionRange, PositionRangeContainer};
use crate::position_reader::Symbol;
use crate::token::{Token, TokenKind};

/// A lexer is an iterator that consumes the FTL sourcecode char-by-char and returns the parsed [Token]s.
pub struct Lexer<SymbolIter: Iterator<Item = Symbol>> {
    /// The source to read the symbols from.
    symbols: Peekable<SymbolIter>,
}

impl<SymbolIter: Iterator<Item = Symbol>> Lexer<SymbolIter> {
    /// Creates a new Lexer from the given symbol iterator.
    pub fn new(symbols: SymbolIter) -> Self {
        Self {
            symbols: symbols.peekable(),
        }
    }

    /// Checks if [Self::symbols] will yield a skip symbol next (See [is_skip_symbol]). If [Self::Symbols] will yield
    /// [None], true is returned. This prevents [skip_skipable_symbols()] from running into an infinite loop.
    fn on_skip_symbol(&mut self) -> bool {
        self.symbols.peek().map_or(true, is_skip_symbol)
    }

    /// Skips all chars of [Lexer.symbols] until the first non-skip symbol is found.
    fn skip_skipable_symbols(&mut self) {
        crate::iter_advance_while(&mut self.symbols, is_skip_symbol);
    }

    /// Goes to the first non-whitespace char in [Lexer.symbols]. If [Lexer.symbols] already stands on a
    /// non-whitespace char, this function does return immediate. Else [Lexer.skip_whitespaces()] is called, which
    /// skips all chars until the first non-whitespace is found.
    pub fn goto_non_skip_symbol(&mut self) {
        // Return early if `self.symbols` already stands on a non-whitespace char
        if !self.on_skip_symbol() {
            return;
        }
        // Search first non-whitespace char
        self.skip_skipable_symbols();
    }

    /// Tokenizes the next symbol from [Lexer::symbols]. Returns [None] if [Lexer::symbols] is drained.
    fn tokenize_next_item(&mut self) -> Option<ParseResult<Token>> {
        self.skip_skipable_symbols();
        // Return None if self.symbols is drained
        Some(match self.symbols.peek()? {
            Symbol { data, .. } if data.is_alphabetic() => {
                // String
                parse_string(self.read_string())
            }
            Symbol { data, .. } if data.is_numeric() => {
                // Number
                parse_number(self.read_number())
            }
            Symbol { data, .. } if is_comment(*data) => {
                // Comment
                let comment = self.read_comment();
                Ok(Token {
                    data: TokenKind::Comment(comment.data),
                    position: comment.position,
                })
            }
            // Not necessary, because goto_non_skip_symbol() skips \r
            /*Symbol {data, ..} if *data == '\r' => {
                // Ignore carriage return
                self.symbols.next();
                None
            },*/
            Symbol { data, .. } if *data == '\n' => {
                // Consume newline
                let Symbol { position, .. } = self.symbols.next().expect(
                    "self.symbols.peek() returned Some(_), but self.symbols.next() returned None",
                );
                Ok(Token {
                    data: TokenKind::EndOfLine,
                    position: position.borrow().into(),
                })
            }
            Symbol { data, .. } if is_special_char(*data) => {
                // Special character
                self.read_special()
            }
            _ => {
                // Consume unknown symbol
                let Symbol { data, position } = self.symbols.next().expect(
                    "self.symbols.peek() returned Some(_), but self.symbols.next() returned None",
                );
                Err(FTLError {
                    kind: FTLErrorKind::IllegalSymbol,
                    msg: format!("Unknown symbol `{}`", data),
                    position: position.borrow().into(),
                })
            }
        })
    }

    fn read_special(&mut self) -> ParseResult<Token> {
        let symbol = self
            .symbols
            .next()
            .expect("read_special called on empty `self.symbols`");
        let symbol_position = symbol.position.borrow().into();
        match symbol.data {
            '+' => Ok(Token {
                data: TokenKind::Plus,
                position: symbol_position,
            }),
            '-' => Ok(Token {
                data: TokenKind::Minus,
                position: symbol_position,
            }),
            '*' => Ok(Token {
                data: TokenKind::Star,
                position: symbol_position,
            }),
            ',' => Ok(Token {
                data: TokenKind::Comma,
                position: symbol_position,
            }),
            '(' => Ok(Token {
                data: TokenKind::OpeningParentheses,
                position: symbol_position,
            }),
            ')' => Ok(Token {
                data: TokenKind::ClosingParentheses,
                position: symbol_position,
            }),
            '{' => Ok(Token {
                data: TokenKind::OpeningCurlyBraces,
                position: symbol_position,
            }),
            '}' => Ok(Token {
                data: TokenKind::ClosingCurlyBraces,
                position: symbol_position,
            }),
            '<' => Ok(Token {
                data: TokenKind::Less,
                position: symbol_position,
            }),
            '.' => Ok(Token {
                data: TokenKind::Dot,
                position: symbol_position,
            }),
            ':' => Ok(Token {
                data: TokenKind::Colon,
                position: symbol_position,
            }),
            '/' => Ok(Token {
                data: TokenKind::Slash,
                position: symbol_position,
            }),
            ';' => Ok(Token {
                data: TokenKind::Semicolon,
                position: symbol_position,
            }),
            '=' => {
                match self.symbols.peek() {
                    Some(Symbol { data: '/', .. }) => self.symbols.next(),
                    _ => {
                        return Ok(Token {
                            data: TokenKind::Equal,
                            position: symbol.position.borrow().into(),
                        })
                    }
                };
                match self.symbols.next() {
                    Some(Symbol {
                        data: '=',
                        position: end_position,
                    }) => Ok(Token {
                        data: TokenKind::NotEqual,
                        position: PositionRange {
                            line: symbol.position.line,
                            column: symbol.position.column..=end_position.column,
                        },
                    }),
                    other => Err(FTLError {
                        kind: FTLErrorKind::IllegalSymbol,
                        msg: format!(
                            "Parsing not-equal token starting with `=/`, but ends with {:?}",
                            other
                        ),
                        // TODO: Better position
                        position: other
                            .map(|symbol| symbol.position.borrow().into())
                            .unwrap_or(PositionRange {
                                line: 1,
                                column: 1..=1,
                            }),
                    }),
                }
            }
            other => Err(FTLError {
                kind: FTLErrorKind::IllegalSymbol,
                msg: format!("Unknown symbol {}", other),
                position: symbol.position.borrow().into(),
            }),
        }
    }

    /// Reads a string from [Lexer::symbols].
    fn read_string(&mut self) -> PositionRangeContainer<String> {
        let first_symbol = self
            .symbols
            .next()
            .expect("read_string called on empty `self.symbols`");
        let mut position = PositionRange::from_start(first_symbol.position.clone());
        let mut identifier = String::from(first_symbol.data);

        loop {
            // Add chars to the identifier as long as there are chars or numeric
            match self.symbols.peek() {
                Some(Symbol {
                    data: symbol,
                    position: symbol_position,
                    ..
                }) if symbol.is_alphanumeric() => {
                    position.set_end(symbol_position);
                    identifier.push(*symbol);
                }
                _ => break,
            }
            self.symbols.next();
        }
        PositionRangeContainer {
            data: identifier,
            position,
        }
    }

    /// Reads a number from [Lexer::symbols].
    fn read_number(&mut self) -> PositionRangeContainer<String> {
        let first_symbol = self
            .symbols
            .next()
            .expect("read_number called on empty `self.symbols`");
        assert!(first_symbol.data.is_numeric());
        let mut position = PositionRange::from_start(first_symbol.position);
        let mut number = String::from(first_symbol.data);
        loop {
            // Add chars to the number as long as there are numeric or a dot
            match self.symbols.peek() {
                Some(Symbol {
                    data: c,
                    position: symbol_position,
                }) if c.is_numeric() => {
                    position.set_end(symbol_position);
                    number.push(*c);
                }
                Some(Symbol {
                    data: c,
                    position: symbol_position,
                }) if *c == '.' => {
                    // Number is a float
                    position.set_end(symbol_position);
                    number.push(*c);
                }
                _ => break,
            }
            self.symbols.next();
        }
        PositionRangeContainer {
            data: number,
            position,
        }
    }

    /// Reads a comment and returns its content.
    fn read_comment(&mut self) -> PositionRangeContainer<String> {
        // Skip introductory comment symbol and save its position
        let first_position = match self.symbols.next() {
            Some(Symbol { data, position }) if is_comment(data) => position,
            _ => panic!("read_comment called on non-comment symbol"),
        };
        // Read comment line
        let comment_symbols: Vec<Symbol> =
            crate::iter_take_while(&mut self.symbols, |symbol| symbol.data != '\n')
                .into_iter()
                .filter(|symbol| symbol.data != '\r')
                .collect();
        // Get the position of the comment. If `comment_symbols` is empty, take `first_position.column` as end.
        let position = PositionRange {
            line: first_position.line,
            column: first_position.column
                ..=comment_symbols
                    .last()
                    .map(|symbol| symbol.position.column)
                    .unwrap_or(first_position.column),
        };
        let comment: String = comment_symbols
            .into_iter()
            .map(|symbol| symbol.data)
            .collect();
        PositionRangeContainer {
            data: comment,
            position,
        }
    }
}

impl<SymbolIter: Iterator<Item = Symbol>> Iterator for Lexer<SymbolIter> {
    type Item = Result<Token, FTLError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize_next_item()
    }
}

/// Parses a string to a keyword or to an identifier.
///
/// # Panics
///
/// Panics if the string is empty.
fn parse_string(string: PositionRangeContainer<String>) -> ParseResult<Token> {
    assert!(!string.data.is_empty(), "Identifier must not be empty");
    // TODO: Extract match statement to HashMap.
    Ok(match string.data.as_str() {
        "def" => Token {
            data: TokenKind::FunctionDefinition,
            position: string.position,
        },
        "extern" => Token {
            data: TokenKind::Extern,
            position: string.position,
        },
        "bitor" => Token {
            data: TokenKind::BitOr,
            position: string.position,
        },
        "bitand" => Token {
            data: TokenKind::BitAnd,
            position: string.position,
        },
        "mod" => Token {
            data: TokenKind::Modulus,
            position: string.position,
        },
        "if" => Token {
            data: TokenKind::If,
            position: string.position,
        },
        "else" => Token {
            data: TokenKind::Else,
            position: string.position,
        },
        "for" => Token {
            data: TokenKind::For,
            position: string.position,
        },
        _ => Token {
            data: TokenKind::Identifier(string.data),
            position: string.position,
        },
    })
}

/// Parses a number to a [TokenType::Number}].
fn parse_number(number: PositionRangeContainer<String>) -> ParseResult<Token> {
    // TODO: Add parsing for other number types.
    let parsed_number: f64 = match number.data.parse() {
        Ok(num) => num,
        Err(e) => {
            return Err(FTLError {
                kind: FTLErrorKind::IllegalSymbol,
                msg: e.to_string(),
                position: number.position,
            })
        }
    };
    Ok(Token {
        data: TokenKind::Number(parsed_number),
        position: number.position,
    })
}

/// Checks if `symbol` starts a comment line.
pub(crate) fn is_comment(symbol: char) -> bool {
    symbol == '#'
}

/// Checks if `symbol` is a special character like `+`, `-`, `=`, `*`.
fn is_special_char(symbol: char) -> bool {
    // TODO: Extract comparison to lazy_static HashSet
    [
        '+', '-', '=', '<', '*', '(', ')', '{', '}', '.', ':', ',', '/', ';',
    ]
    .contains(&symbol)
}

/// Returns whether `symbol` should be skipped or not.
///
/// [Symbol]s to be skipped are:
/// - whitespaces
/// - carriage returns (`\r`)
fn is_skip_symbol(symbol: &Symbol) -> bool {
    ['\r', ' ', '\t'].contains(&symbol.data)
}

#[cfg(test)]
mod test {
    use crate::position_reader::PositionReader;

    use super::*;

    /// Tests [Lexer::goto_non_skip_symbol].
    #[test]
    fn lexer_goto_non_skip_symbol_test() {
        let sourcecode = " ab c  ";
        let position_reader = PositionReader::new(sourcecode.chars());
        let mut lexer = Lexer::new(position_reader);
        lexer.goto_non_skip_symbol();
        assert_eq!(lexer.symbols.next().map(|symbol| symbol.data), Some('a'));
        lexer.goto_non_skip_symbol();
        assert_eq!(lexer.symbols.next().map(|symbol| symbol.data), Some('b'));
        lexer.goto_non_skip_symbol();
        assert_eq!(lexer.symbols.next().map(|symbol| symbol.data), Some('c'));
        lexer.goto_non_skip_symbol();
        assert_eq!(lexer.symbols.next().map(|symbol| symbol.data), None);
        // Verify that multiple calls to lexer.goto_non_skip_symbol() after the first None do not result in an infinite
        // loop
        lexer.goto_non_skip_symbol();
        assert_eq!(lexer.symbols.next().map(|symbol| symbol.data), None);
    }
}
