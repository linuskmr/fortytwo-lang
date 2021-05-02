mod token;
use std::io;

pub use token::{Token, TokenType};

#[derive(Debug)]
enum Symbol {
    Char(char),
    Whitespace,
    Newline,
    /// End Of File
    EOF,
}

pub struct Lexer<R: io::Read> {
    /// The source to read from.
    reader: R,
    /// The current char.
    current_char: Symbol,
}

impl<R: io::Read> Lexer<R> {
    /// Creates a new Lexer with the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_char: Symbol::Whitespace, // Necessary for read_until_whitespace to start working at all
        }
    }

    /// Reads the next symbol from self.reader into self.current_char.
    fn read_next_symbol(&mut self) {
        let mut buffer = [0u8; 1];
        self.current_char = match self.reader.read(&mut buffer).unwrap() {
            0 => Symbol::EOF,
            _ if (buffer[0] as char).is_whitespace() => Symbol::Whitespace,
            _ if (buffer[0] as char) == '\n' => Symbol::Newline,
            _ => Symbol::Char(buffer[0] as char),
        };
    }

    /// Discards all whitespace symbols until a non-whitespace symbol is found.
    /// The symbol is saved in self.current_char.
    ///
    /// # Errors
    ///
    /// If an EOF is present at self.reader while reading, this method
    /// returns an Err.
    fn read_until_not_whitespace(&mut self) -> Result<(), ()> {
        loop {
            match self.current_char {
                Symbol::EOF => return Err(()),
                Symbol::Char(_) => return Ok(()),
                Symbol::Whitespace | Symbol::Newline => (), // Ignore whitespace
            };
            self.read_next_symbol();
        }
    }

    fn tokenize_next_item(&mut self) -> Option<TokenType> {
        if let Symbol::Char(current_char) = self.current_char {
            // Read identifier
            if current_char.is_alphabetic() {
                let identifier = self.read_identifier();
                let token = parse_identifier(identifier);
                Some(token)
            } else if current_char.is_numeric() {
                // Read number
                let number_string = self.read_number_string();
                let number = parse_float(number_string);
                Some(TokenType::Number(number))
            } else if current_char == '#' {
                // Ignore comment
                self.ignore_comment();
                self.tokenize_next_item()
            } else {
                // Other
                let other = Some(TokenType::Other(current_char));
                self.read_next_symbol();
                other
            }
        } else {
            None
        }
    }

    /// Reads an identifier to a string.
    ///
    /// # Panics
    ///
    /// Panics if self.current_char is not Symbol::Char or its content is non-alphabetic.
    fn read_identifier(&mut self) -> String {
        // Create identifier with current char
        let mut identifier = match self.current_char {
            Symbol::Char(c) if c.is_alphabetic() => String::from(c),
            _ => panic!("read_identifier called with non-alphabetic self.current_char"),
        };

        // Add chars to the identifier as long as there are chars
        loop {
            self.read_next_symbol();
            match self.current_char {
                Symbol::Char(c) if c.is_alphanumeric() => identifier.push(c),
                _ => break,
            }
        }
        identifier
    }

    /// Reads an number to a string.
    ///
    /// # Panics
    ///
    /// Panics if self.current_char is not Symbol::Char or its content is non-numeric.
    fn read_number_string(&mut self) -> String {
        // Create number string with current char
        let mut number_string = match self.current_char {
            Symbol::Char(c) if c.is_numeric() => String::from(c),
            _ => panic!("read_identifier called with non-numeric self.current_char"),
        };

        // Add chars to the identifier as long as there are numeric
        loop {
            self.read_next_symbol();
            match self.current_char {
                Symbol::Char(c) if c.is_numeric() => number_string.push(c),
                Symbol::Char(c) if c == '.' => number_string.push(c), // Future: Number is now of type f64, not int (Not considered yet)
                _ => break,
            }
        }
        number_string
    }

    /// Skips a comment line.
    fn ignore_comment(&mut self) {
        loop {
            self.read_next_symbol();
            match self.current_char {
                Symbol::Char(c) if c == '\n' => break,
                Symbol::EOF => break,
                _ => (),
            }
        }
    }
}

/// Parses an identifier to a keyword or to an identifier.
///
/// # Panics
///
/// Panic if the identifier is empty.
fn parse_identifier(identifier: String) -> TokenType {
    assert!(identifier.len() >= 1, "Identifier is empty");
    match identifier.as_str() {
        "def" => TokenType::Def,
        "extern" => TokenType::Extern,
        _ => TokenType::Identifier(identifier),
    }
}

/// Parses float_string to an f64.
///
/// # Panics
///
/// Panics if float_string is not parsable.
fn parse_float(float_string: String) -> f64 {
    float_string.parse().unwrap()
}

impl<R: io::Read> Iterator for Lexer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_until_not_whitespace().ok()?;
        self.tokenize_next_item().and_then(|token_type| Some(Token {
            token_type,
        }))
    }
}