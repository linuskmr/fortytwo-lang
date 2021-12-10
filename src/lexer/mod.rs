mod error;
use crate::position_container::PositionContainer;
use crate::position_reader::Symbol;
use crate::token::{Token, TokenKind};
use miette::{IntoDiagnostic, NamedSource, SourceSpan};
use std::iter::Peekable;
use std::sync::Arc;

/// A lexer is an iterator that consumes the FTL sourcecode char-by-char and returns the parsed [Token]s.
pub struct Lexer<SymbolIter: Iterator<Item = Symbol>> {
    /// The source to read the symbols from.
    symbols: Peekable<SymbolIter>,

    named_source: Arc<NamedSource>,
}

impl<SymbolIter: Iterator<Item = Symbol>> Lexer<SymbolIter> {
    /// Creates a new Lexer from the given symbol iterator.
    pub fn new(symbols: SymbolIter, named_source: Arc<NamedSource>) -> Self {
        Self {
            symbols: symbols.peekable(),
            named_source,
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
    fn tokenize_next_item(&mut self) -> Option<miette::Result<Token>> {
        self.skip_skipable_symbols();
        // Return None if self.symbols is drained
        Some(match self.symbols.peek()? {
            Symbol { data, .. } if data.is_alphabetic() => {
                // String
                let read_string = self.read_string();
                self.parse_string(read_string)
            }
            Symbol { data, .. } if data.is_numeric() => {
                // Number
                let read_number = self.read_number();
                self.parse_number(read_number)
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
                    position,
                })
            }
            Symbol { data, .. } if is_special_char(*data) => {
                // Special character
                self.read_special()
            }
            _ => {
                // Consume unknown symbol
                let Symbol { position, .. } = self.symbols.next().expect(
                    "self.symbols.peek() returned Some(_), but self.symbols.next() returned None",
                );
                Err(error::UnknownSymbol {
                    src: self.named_source.clone(),
                    err_span: position,
                }
                .into())
            }
        })
    }

    fn read_special(&mut self) -> miette::Result<Token> {
        let symbol = self
            .symbols
            .next()
            .expect("read_special called on empty `self.symbols`");
        match symbol.data {
            '+' => Ok(Token {
                data: TokenKind::Plus,
                position: symbol.position,
            }),
            '-' => Ok(Token {
                data: TokenKind::Minus,
                position: symbol.position,
            }),
            '*' => Ok(Token {
                data: TokenKind::Star,
                position: symbol.position,
            }),
            ',' => Ok(Token {
                data: TokenKind::Comma,
                position: symbol.position,
            }),
            '(' => Ok(Token {
                data: TokenKind::OpeningParentheses,
                position: symbol.position,
            }),
            ')' => Ok(Token {
                data: TokenKind::ClosingParentheses,
                position: symbol.position,
            }),
            '{' => Ok(Token {
                data: TokenKind::OpeningCurlyBraces,
                position: symbol.position,
            }),
            '}' => Ok(Token {
                data: TokenKind::ClosingCurlyBraces,
                position: symbol.position,
            }),
            '<' => Ok(Token {
                data: TokenKind::Less,
                position: symbol.position,
            }),
            '.' => Ok(Token {
                data: TokenKind::Dot,
                position: symbol.position,
            }),
            ':' => Ok(Token {
                data: TokenKind::Colon,
                position: symbol.position,
            }),
            '/' => Ok(Token {
                data: TokenKind::Slash,
                position: symbol.position,
            }),
            ';' => Ok(Token {
                data: TokenKind::Semicolon,
                position: symbol.position,
            }),
            '=' => {
                match self.symbols.peek() {
                    Some(Symbol { data: '/', .. }) => self.symbols.next(),
                    _ => {
                        return Ok(Token {
                            data: TokenKind::Equal,
                            position: symbol.position,
                        })
                    }
                };
                match self.symbols.next() {
                    Some(Symbol { data: '=', .. }) => Ok(Token {
                        data: TokenKind::NotEqual,
                        position: SourceSpan::new(symbol.position.offset().into(), 3.into()),
                    }),
                    _ => Err(error::IllegalNonEqualToken {
                        src: self.named_source.clone(),
                        err_span: (symbol.position.offset() + 2, 1).into(),
                    })?,
                }
            }
            _other => Err(error::UnknownSymbol {
                src: self.named_source.clone(),
                err_span: symbol.position,
            })?,
        }
    }

    /// Reads a string from [Lexer::symbols].
    fn read_string(&mut self) -> PositionContainer<String> {
        let first_symbol = self
            .symbols
            .next()
            .expect("read_string called on empty `self.symbols`");
        let mut identifier = String::from(first_symbol.data);
        loop {
            // Add chars to the identifier as long as there are chars or numeric
            match self.symbols.peek() {
                Some(Symbol { data: symbol, .. }) if symbol.is_alphanumeric() => {
                    identifier.push(*symbol)
                }
                _ => break,
            }
            self.symbols.next();
        }
        PositionContainer {
            position: SourceSpan::new(
                first_symbol.position.offset().into(),
                identifier.len().into(),
            ),
            data: identifier,
        }
    }

    /// Reads a number from [Lexer::symbols].
    fn read_number(&mut self) -> PositionContainer<String> {
        let first_symbol = self
            .symbols
            .next()
            .expect("read_number called on empty `self.symbols`");
        assert!(first_symbol.data.is_numeric());
        let mut number = String::from(first_symbol.data);
        loop {
            // Add chars to the number as long as there are numeric or a dot
            match self.symbols.peek() {
                Some(Symbol { data, .. }) if data.is_numeric() => number.push(*data),
                Some(Symbol { data, .. }) if *data == '.' => number.push(*data), // Number is a float
                _ => break,
            }
            self.symbols.next();
        }
        PositionContainer {
            position: SourceSpan::new(first_symbol.position.offset().into(), number.len().into()),
            data: number,
        }
    }

    /// Reads a comment and returns its content.
    fn read_comment(&mut self) -> PositionContainer<String> {
        // Skip introductory comment symbol and save its position
        let first_position = match self.symbols.next() {
            Some(Symbol { data, position }) if is_comment(data) => position,
            _ => panic!("read_comment called on non-comment symbol"),
        };
        // Read comment line
        let comment: String =
            crate::iter_take_while(&mut self.symbols, |symbol| symbol.data != '\n')
                .into_iter()
                .map(|symbol| symbol.data)
                .filter(|&c| c != '\r')
                .collect();
        PositionContainer {
            position: SourceSpan::new(first_position.offset().into(), comment.len().into()),
            data: comment,
        }
    }

    /// Parses a string to a keyword or to an identifier.
    ///
    /// # Panics
    ///
    /// Panics if the string is empty.
    fn parse_string(&self, string: PositionContainer<String>) -> miette::Result<Token> {
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

    /// Parses a number to a [TokenType::Number].
    fn parse_number(&self, number: PositionContainer<String>) -> miette::Result<Token> {
        // TODO: Add parsing for other number types.
        let parsed_number: f64 = match number.data.parse().into_diagnostic() {
            Ok(num) => num,
            Err(_err) => {
                return Err(error::ParseNumberError {
                    src: self.named_source.clone(),
                    err_span: number.position,
                }
                .into())
            }
        };
        Ok(Token {
            data: TokenKind::Number(parsed_number),
            position: number.position,
        })
    }
}

impl<SymbolIter: Iterator<Item = Symbol>> Iterator for Lexer<SymbolIter> {
    type Item = miette::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize_next_item()
    }
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
