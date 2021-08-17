use crate::position_container::{PositionRange, PositionRangeContainer, Position};
use crate::position_reader::{PositionReader, Symbol};
use crate::token::{Token, TokenType};
use crate::error::{FTLError, FTLErrorKind};
use std::iter::Peekable;
use std::borrow::Borrow;


/// A lexer is an iterator that consumes the FTL sourcecode char-by-char and returns the parsed [Token]s.
pub struct Lexer<SymbolIter: Iterator<Item=char>> {
    /// The source to read the symbols from.
    symbols: Peekable<PositionReader<SymbolIter>>
}

impl<SymbolIter: Iterator<Item=char>> Lexer<SymbolIter> {
    /// Creates a new Lexer from the given symbol iterator.
    pub fn new(symbols: SymbolIter) -> Self {
        Self { symbols: PositionReader::new(symbols).peekable() }
    }

    /// Checks if [Lexer.symbols] stands on a skip symbol.
    fn on_skip_symbol(&mut self) -> bool {
        is_skip_symbol(self.symbols.peek())
    }

    /// Skips all chars of [Lexer.symbols] until the first non-skip symbol is found.
    fn skip_skipable_symbols(&mut self) {
        advance_iter_while(&mut self.symbols, is_skip_symbol);
    }

    /// Goes to the first non-whitespace char in [Lexer.symbols]. If [Lexer.symbols] already stands on a
    /// non-whitespace char, this function does return immediate. Else [Lexer.skip_whitespaces()] is called, which
    /// skips all chars until the first non-whitespace.
    ///
    /// ```
    /// use ftllib::lexer::Lexer;
    /// let lexer = Lexer::new();
    /// ```
    pub(crate) fn goto_non_skipable_symbol(&mut self) {
        // Return early if `self.symbols` already stands on a non-whitespace char
        if self.on_skip_symbol() {
            return
        }
        // Search first non-whitespace char
        self.skip_skipable_symbols();
    }

    /// Tokenizes the next symbol from [Lexer::symbols]. Returns `None` if the symbol can be ignored (e.g. comment or
    /// carriage return). That means that this function does not always return all `Some`'s first and than always None,
    /// like an iterator does. To check if there are no more symbols to tokenize, check if [self.symbols.peek()] is
    /// None.
    /// Furthermore this function assumes that [Lexer::symbols] does not yield a whitespace character, so you have to
    /// check that before calling this function. Otherwise this function will return an `Err`, because of an unknown
    /// Symbol.
    fn tokenize_next_item(&mut self) -> Option<Result<Token, FTLError>> {
        // Return None if self.symbols is drained
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
                self.read_comment();
                None
            },
            Symbol {data, ..} if *data == '\r' => {
                // Ignore carriage return
                self.symbols.next();
                None
            },
            Symbol {data, position} if *data == '\n' => {
                // Newline
                Some(Ok(Token { data:TokenType::EndOfLine, position: position.into() }))
            },
            Symbol {data, ..} if is_special_char(*data) => {
                // Special character
                Some(self.read_special())
            },
            Symbol { data, position } => {
                // Unknown symbol
                Some(Err(FTLError {
                    kind: FTLErrorKind::IllegalSymbol,
                    msg: format!("Unknown symbol `{}`", data),
                    position: position.into()
                }))
            }
        }
    }

    fn read_special(&mut self) -> Result<Token, FTLError> {
        let symbol = self.symbols.next().expect("read_special called on empty `self.symbols`");
        match symbol.data {
            '+' => Ok(Token { data: TokenType::Plus, position: symbol.position.borrow().into() }),
            '-' => Ok(Token { data: TokenType::Minus, position: symbol.position.borrow().into() }),
            '*' => Ok(Token { data: TokenType::Star, position: symbol.position.borrow().into() }),
            ',' => Ok(Token { data: TokenType::Comma, position: symbol.position.borrow().into() }),
            '(' => Ok(Token { data: TokenType::OpeningParentheses, position: symbol.position.borrow().into() }),
            ')' => Ok(Token { data: TokenType::ClosingParentheses, position: symbol.position.borrow().into() }),
            '<' => Ok(Token { data: TokenType::Less, position: symbol.position.borrow().into() }),
            '=' => {
                match self.symbols.peek() {
                    Some(Symbol {data: '/', ..}) => self.symbols.next(),
                    _ => return Ok(Token {data: TokenType::Equal, position: symbol.position.borrow().into()}),
                };
                match self.symbols.next() {
                    Some(Symbol {data: '=', position: end_position}) => Ok(Token {
                        data: TokenType::NotEqual,
                        position: PositionRange {
                            line: symbol.position.line,
                            column: symbol.position.column..=end_position.column
                        }
                    }),
                    other => Err(FTLError {
                        kind: FTLErrorKind::IllegalSymbol,
                        msg: format!("Parsing not-equal token starting with `=/`, but ends with {:?}", other),
                        // TODO: Better position
                        position: other.map(|symbol| symbol.position.borrow().into()).unwrap_or(PositionRange {
                            line: 1, column: 1..=1
                        })
                    })
                }
            }
            other => Err(FTLError {
                kind: FTLErrorKind::IllegalSymbol,
                msg: format!("Unknown symbol {}", other),
                position: symbol.position.borrow().into()
            })
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
    fn read_comment(&mut self) -> PositionRangeContainer<String> {
        // Skip introductory comment symbol and save its position
        let first_position = match self.symbols.next() {
            Some(Symbol {data, position}) if is_comment(data) => position,
            _ => panic!("read_comment called on non-comment symbol"),
        };
        // Read comment line
        let comment_symbols: Vec<Symbol> = self.symbols
            .take_while(|symbol| symbol.data != '\n')
            .filter(|symbol| symbol.data != '\r')
            .collect();
        // Get the position of the comment. If `comment_symbols` is empty, take `first_position.column` as end.
        let position = PositionRange {
            line: first_position.line,
            column: first_position.column..=comment_symbols.last()
                .map(|symbol| symbol.position.column)
                .unwrap_or(first_position.column)
        };
        let comment: String = comment_symbols.into_iter().map(|symbol| symbol.data).collect();
        PositionRangeContainer{ data: comment, position}
    }
}

/// Parses a string to a keyword or to an identifier.
///
/// # Panics
///
/// Panics if the string is empty.
fn parse_string(string: PositionRangeContainer<String>) -> Result<Token, FTLError> {
    assert!(!string.data.is_empty(), "Identifier must not be empty");
    // TODO: Extract match statement to HashMap.
    Ok(match string.data.as_str() {
        "def" => Token { data: TokenType::Def, position: string.position },
        "extern" => Token { data: TokenType::Extern, position: string.position },
        "bitor" => Token { data: TokenType::BitOr, position: string.position },
        "bitand" => Token { data: TokenType::BitAnd, position: string.position },
        "mod" => Token { data: TokenType::Modulus, position: string.position },
        _ => Token { data: TokenType::Identifier(string.data), position: string.position },
    })
}

/// Parses a number to a [TokenType::Number}].
fn parse_number(number: PositionRangeContainer<String>) -> Result<Token, FTLError> {
    // TODO: Add parsing for other number types.
    let parsed_number: f64 = match number.data.parse() {
        Ok(num) => num,
        Err(e) => return Err(FTLError {
            kind: FTLErrorKind::IllegalSymbol,
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
    symbol == '+' || symbol == '-' || symbol == '=' || symbol == '*' || symbol == '(' || symbol == ')'
}

impl<SymbolIter: Iterator<Item=char>> Iterator for Lexer<SymbolIter> {
    type Item = Result<Token, FTLError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Make self.symbols not return a whitespace, which is assumed by `self.tokenize_next_item()`
            self.goto_non_skipable_symbol();
            // If self.symbols is drained, we will return here
            self.symbols.peek()?;
            // Tokenize returned a token? Then return it, otherwise try again
            if let Some(token) = self.tokenize_next_item() {
                return Some(token)
            }
        }
    }
}

/// Advances the `iterator` while `condition` returns true.
fn advance_iter_while<Iter, Func, Elem>(iterator: &mut Peekable<Iter>, condition: Func)
    where Iter: Iterator<Item=Elem>, Func: Fn(Option<&Elem>) -> bool
{
    loop {
        let element = iterator.peek();
        if !condition(element) {
            break
        };
        iterator.next();
    }
}

fn is_skip_symbol(symbol: Option<&Symbol>) -> bool {
    symbol.map(|symbol| symbol.data.is_whitespace() || symbol.data == '\n' || symbol.data == '\r')
        .unwrap_or(false)
}
