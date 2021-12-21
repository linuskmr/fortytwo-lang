use std::iter::{Enumerate, Peekable};
use std::sync::Arc;

use miette::{NamedSource, SourceSpan};

use crate::position::{PositionContainer, PositionRange};
use crate::token::{Token, TokenKind};

mod error;

/// A lexer is an iterator that consumes the FTL sourcecode char-by-char and returns the parsed [Token]s.
pub struct Lexer<LetterIter: Iterator<Item=char>> {
    /// The source to read the letters from.
    letters: Peekable<Enumerate<LetterIter>>,

    named_source: Arc<NamedSource>,
}

impl<LetterIter: Iterator<Item=char>> Lexer<LetterIter> {
    /// Creates a new Lexer from the given char iterator.
    pub fn new(symbols: LetterIter, named_source: Arc<NamedSource>) -> Self {
        Self { letters: symbols.enumerate().peekable(), named_source }
    }

    /// Checks if [Self::letters] will yield a letter nex, that should be ignored, i.e. skipped.
    /// If [Self::letters] will yield [None], true is returned.
    /// This prevents [skip_skipable_symbols()]from running into an infinite loop.
    fn on_ignore_letter(&mut self) -> bool {
        self.letters.peek().map_or(true, |&(_, letter) | is_skip_letter(letter))
    }

    /// Skips all letters of [Self::letters] until the first normal letter is found.
    fn skip_ignore_letters(&mut self) {
        crate::iter_advance_while(&mut self.letters, |&(_, letter)| is_skip_letter(letter));
    }

    /// Goes to the first char in [Self::symbols] that should not be ignored. If [Self::symbols] already stands on a
    /// normal char, this function does return immediate. Else [Lexer::skip_ignore_letters()] is called, which
    /// skips all letters until the first normal letter is found.
    pub fn goto_normal_letter(&mut self) {
        // Return early if `self.letters` already stands on a normal char
        if !self.on_ignore_letter() {
            return;
        }
        // Search first normal char
        self.skip_ignore_letters();
    }

    /// Tokenizes the next symbol from [Lexer::letters]. Returns [None] if [Lexer::letters] is drained.
    fn tokenize_next_item(&mut self) -> Option<miette::Result<Token>> {
        self.skip_ignore_letters();
        // Return None if self.symbols is drained
        let (position, letter) = self.letters.peek()?.clone();
        let symbol = match letter {
            letter if letter.is_alphabetic() => {
                // String
                let read_string = self.read_string();
                self.parse_string(read_string)
            }
            letter if letter.is_numeric() => {
                // Number
                let read_number = self.read_number();
                self.parse_number(read_number)
            }
            letter if is_comment(letter) => {
                // Comment
                let (position, comment) = self.read_comment();
                Ok((position, TokenKind::Comment(comment)))
            }
            // Not necessary, because goto_non_skip_symbol() skips \r
            /*Symbol {data, ..} if *data == '\r' => {
                // Ignore carriage return
                self.symbols.next();
                None
            },*/
            '\n' => {
                // Consume newline
                self.letters.next();
                Ok((PositionRange { start: position, length: letter.len_utf8() }, TokenKind::EndOfLine))
            }
            letter if is_special_char(letter) => {
                // Special character
                self.read_special()
            }
            _ => {
                // Consume unknown symbol
                Err(error::UnknownSymbol {
                    src: self.named_source.clone(),
                    err_span: SourceSpan::new(position.into(), letter.len_utf8().into()),
                }
                .into())
            }
        };
        Some(symbol)
    }

    fn read_special(&mut self) -> miette::Result<Token> {
        let (position, letter) = self.letters.next().unwrap();
        let position_range = PositionRange { start: position, length: letter.len_utf8() };
        match letter {
            '+' => Ok((position_range, TokenKind::Plus)),
            '-' => Ok((position_range, TokenKind::Minus)),
            '*' => Ok((position_range, TokenKind::Star)),
            ',' => Ok((position_range, TokenKind::Comma)),
            '(' => Ok((position_range, TokenKind::OpeningParentheses)),
            ')' => Ok((position_range, TokenKind::ClosingParentheses)),
            '{' => Ok((position_range, TokenKind::OpeningCurlyBraces)),
            '}' => Ok((position_range, TokenKind::ClosingCurlyBraces)),
            '<' => Ok((position_range, TokenKind::Less)),
            '.' => Ok((position_range, TokenKind::Dot)),
            ':' => Ok((position_range, TokenKind::Colon)),
            '/' => Ok((position_range, TokenKind::Slash)),
            ';' => Ok((position_range, TokenKind::Semicolon)),
            '=' => {
                match self.letters.peek() {
                    Some((_, '/')) => self.letters.next(),
                    _ => return Ok((position_range, TokenKind::Equal)),
                };
                match self.letters.next() {
                    Some((equal_position, '=')) => Ok((
                        PositionRange { start: position, length: equal_position - position },
                        TokenKind::NotEqual
                    )),
                    Some((position, letter)) => Err(error::IllegalSymbol {
                        src: self.named_source.clone(),
                        err_span: (position, letter.len_utf8()).into(),
                    })?,
                    None => Err(error::IllegalSymbol {
                        src: self.named_source.clone(),
                        err_span: (position+1, 1).into(),
                    })?,
                }
            },
            _ => Err(error::UnknownSymbol {
                src: self.named_source.clone(),
                err_span: (position, letter.len_utf8()).into(),
            })?,
        }
    }

    /// Reads a string from [Lexer::symbols].
    fn read_string(&mut self) -> PositionContainer<String> {
        let (start_position, letter) = self.letters.next().unwrap();
        assert!(letter.is_alphabetic());
        let mut identifier = String::from(letter);
        loop {
            // Add chars to the identifier as long as there are chars or numeric
            match self.letters.peek() {
                Some((_, letter)) if letter.is_alphanumeric()  => identifier.push(*letter),
                _ => break,
            }
            self.letters.next();
        }
        (PositionRange { start: start_position, length: identifier.len() }, identifier)
    }

    /// Reads a number from [Lexer::symbols].
    fn read_number(&mut self) -> PositionContainer<String> {
        let (start_position, letter) = self.letters.next().unwrap();
        assert!(letter.is_numeric());
        let mut number = String::from(letter);
        loop {
            // Add chars to the number as long as there are numeric or a dot
            match self.letters.peek() {
                Some((_, letter)) if letter.is_numeric() => number.push(*letter),
                Some((_, '.')) => number.push(letter), // Number is a float
                _ => break,
            }
            self.letters.next();
        }
        (PositionRange { start: start_position, length: number.len() }, number)
    }

    /// Reads a comment and returns its content.
    fn read_comment(&mut self) -> PositionContainer<String> {
        // Skip introductory comment symbol and save its position
        let (first_position, _) = self.letters.next().unwrap();
        // Because we may encounter newlines and then skip some ignore letters, we cannot simply determine the length
        // of the comment by the parsed comment, but have to manually update the last position.
        let mut last_position = first_position;
        let mut comment = String::new();
        // Read letters and save them into comment
        loop {
            match self.letters.peek() {
                Some((_, '\n')) => {
                    // Detected linebreak. Now check if the next line is also a comment.
                    // If yes, continue parsing the next line
                    self.letters.next(); // Consume \n
                    self.goto_normal_letter(); // Skip possible leading whitespaces
                    match self.letters.peek().map(|symbol| symbol.1) {
                        Some(letter) if is_comment(letter) => (), // Is comment. Continue parsing
                        _ => break, // Either none or not a comment. Break parsing
                    };
                    comment.push('\n');
                }
                // Push next letter to comment
                Some(&(position, letter)) => {
                    comment.push(letter);
                    last_position = position;
                },
                // File read to end
                None => break,
            }
        }
        (PositionRange { start: first_position, length: last_position - first_position}, comment)
    }

    /// Parses a string to a keyword or to an identifier.
    ///
    /// # Panics
    ///
    /// Panics if the string is empty.
    fn parse_string(&self, (position, string): PositionContainer<String>) -> miette::Result<Token> {
        assert!(!string.is_empty(), "Identifier must not be empty");
        // TODO: Extract match statement to HashMap.
        Ok(match string.as_str() {
            "def" => (position, TokenKind::FunctionDefinition),
            "extern" => (position, TokenKind::Extern),
            "bitor" => (position, TokenKind::BitOr),
            "bitand" => (position, TokenKind::BitAnd),
            "mod" => (position, TokenKind::Modulus),
            "if" => (position, TokenKind::If),
            "else" => (position, TokenKind::Else),
            "for" =>(position, TokenKind::For),
            _ => (position, TokenKind::Identifier(string)),
        })
    }

    /// Parses a number to a [TokenType::Number].
    fn parse_number(&self, (position, number_str): PositionContainer<String>) -> miette::Result<Token> {
        // TODO: Add parsing for other number types.
        let number: f64 = match number_str.parse() {
            Ok(num) => num,
            Err(_) => return Err(error::ParseNumberError {
                src: self.named_source.clone(),
                err_span: position.into(),
            })?
        };
        Ok((position, TokenKind::Float(number)))
    }
}

impl<LetterIter: Iterator<Item=char>> Iterator for Lexer<LetterIter> {
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

/// Returns whether `letter` should be skipped or not.
///
/// Letters to be skipped are:
/// - whitespaces and tabulators
/// - carriage returns (`\r`)
fn is_skip_letter(letter: char) -> bool {
    ['\r', ' ', '\t'].contains(&letter)
}
