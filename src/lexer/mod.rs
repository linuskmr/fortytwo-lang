//! Analyzes the sourcecode char-by-char and converts it to [Token]s.

mod error;

use std::iter::Peekable;
use std::ops::Deref;
pub use error::Error;
use crate::source::{PositionContainer, SourcePositionRange, Symbol};
use crate::Token;
use crate::token::TokenKind;

pub type ParseResult = Result<Token, Error>;


/// Analyzes the sourcecode char-by-char and converts it to [Token]s.
///
/// A lexer is the first phase of a compiler. It analyses the text of the sourcecode and builds
/// [Token]s, like [Identifier("foo")], [Number(42)] or [Operator::Plus]. The lexer is not aware of the meaning of
/// the tokens, it just builds them.
pub struct Lexer<T> where
	T: Iterator<Item = Symbol>
{
	/// Iterator over [Symbol]s of the source code.
	symbols: Peekable<T>,
}

impl<T> Lexer<T> where
	T: Iterator<Item = Symbol>
{
	/// Creates a [Lexer] from the given [Symbol] iterator.
	pub fn new(symbols: T) -> Self {
		Self {
			symbols: symbols.peekable(),
		}
	}

	/// Checks whether [Self::symbols] is going to yield a whitespace next. This is used for skipping irrelevant letters.
	/// If [Self::symbols] is going to yield [None], `false` is returned. This prevents [Self::skip_whitespaces()]
	/// from running into an infinite loop.
	fn on_whitespace(&mut self) -> bool {
		self.symbols.peek()
			.map(|symbol| symbol.is_whitespace())
			.unwrap_or(false)
	}

	/// Skips all whitespace symbols until the first "normal" (non-whitespace) letter is found.
	fn skip_whitespaces(&mut self) {
		while self.on_whitespace() {
			self.symbols.next();
		}
	}

	/// Tokenizes the next symbol from [Lexer::letters]. Returns [None] if [Lexer::letters] is drained.
	fn tokenize_next_item(&mut self) -> Option<ParseResult> {
		self.skip_whitespaces();
		// Returns `None` if `self.symbols` is drained
		let symbol = self.symbols.peek()?.clone();

		let token = match symbol {
			symbol if symbol.is_alphabetic() => {
				// String
				let read_string = self.read_string();
				parse_string(read_string)
			}
			symbol if symbol.is_numeric() => {
				// Number
				let number = self.read_number();
				parse_number(number)
			}
			symbol if is_comment(*symbol) => {
				// Comment
				let comment = self.read_comment();
				Ok(Token::new(TokenKind::Comment((*comment).clone()), comment.position))
			}
			/*symbol if symbol == '\n' => {
				// Consume newline
				assert_eq!(self.letters.next().map(&|(_, letter)| letter), Some('\n'));
				Ok(Token {
					kind: TokenKind::EndOfLine,
					position: Position::from_start_len(position, letter.len_utf8()),
				})
			}*/
			symbol if is_special_char(*symbol) => {
				// Special character
				self.read_special()
			}
			_ => {
				// Consume unknown symbol
				self.symbols.next();
				Err(Error::UnknownSymbol(symbol))
			},
		};
		Some(token)
	}

	fn read_string(&mut self) -> PositionContainer<String> {
		let mut string = String::new();
		let mut position = self.symbols.peek().unwrap().position.clone();
		while let Some(symbol) = self.symbols.peek().cloned() {
			if symbol.is_alphanumeric() || *symbol == '_' {
				string.push(*symbol);
				self.symbols.next();
				position.position.end = symbol.position.position.end;
			} else {
				break;
			}
		}
		PositionContainer::new(string, position)
	}

	// Reads a number from [Lexer::symbols].
	fn read_number(&mut self) -> PositionContainer<String> {
		let first_symbol = self.symbols.next().unwrap();
		let mut position = first_symbol.position.clone();
		let mut number = String::from(*first_symbol);
		loop {
			// Add chars to the number as long as there are numeric or a dot
			match self.symbols.peek() {
				Some(symbol) if symbol.is_numeric() => {
					number.push(**symbol);
					position.position.end = symbol.position.position.end;
				},
				Some(symbol) if **symbol == '.' => {
					number.push('.');
					position.position.end = symbol.position.position.end;
				},
				_ => break,
			}
			self.symbols.next();
		}
		PositionContainer::new(
			number,
			position,
		)
	}

	fn read_special(&mut self) -> ParseResult {
		let symbol = self.symbols.next().unwrap();
		let position = symbol.position.clone();
		match *symbol {
			'+' => Ok(Token::new(TokenKind::Plus, position)),
			'-' => Ok(Token::new(TokenKind::Minus, position)),
			'*' => Ok(Token::new(TokenKind::Star, position)),
			',' => Ok(Token::new(TokenKind::Comma, position)) ,
			'(' => Ok(Token::new(TokenKind::OpeningParentheses, position)),
			')' => Ok(Token::new(TokenKind::ClosingParentheses, position)),
			'{' => Ok(Token::new(TokenKind::OpeningCurlyBraces, position)),
			'}' => Ok(Token::new(TokenKind::ClosingCurlyBraces, position)),
			'<' => Ok(Token::new(TokenKind::Less, position)),
			'.' => Ok(Token::new(TokenKind::Dot, position)),
			':' => Ok(Token::new(TokenKind::Colon, position)),
			'/' => Ok(Token::new(TokenKind::Slash, position)),
			';' => Ok(Token::new(TokenKind::Semicolon, position)),
			'[' => Ok(Token::new(TokenKind::OpeningSquareBrackets, position)),
			']' => Ok(Token::new(TokenKind::ClosingSquareBrackets, position)),
			'=' => {
				match self.symbols.peek() {
					// Read token is `=/` so far
					Some(symbol) if **symbol == '/' => self.symbols.next(),
					// Ok, only a single `=` as token
					_ => return Ok(Token::new(TokenKind::Equal, position)),
				};
				match self.symbols.next() {
					// Read token is `=/=`, i.e. not equal
					Some(symbol) if *symbol == '=' => Ok(Token::new(
						TokenKind::NotEqual,
						position,
					)),
					// Illegal token `=/...`
					symbol => Err(Error::IllegalSymbol(symbol))?,
				}
			},
			_ => Err(Error::IllegalSymbol(Some(symbol))),
		}
	}

	/// Reads a comment and returns its content.
	fn read_comment(&mut self) -> PositionContainer<String> {
		// Skip comment symbol
		let mut postion = self.symbols.next().unwrap().position;

		let mut comment = String::new();
		// Read letters and save them into comment
		loop {
			match self.symbols.peek() {
				Some(symbol) if **symbol == '\n' => {
					// Detected newline. Check if the next line is also a comment. If yes, continue parsing the next line
					self.symbols.next(); // Consume \n
					self.skip_whitespaces(); // Skip possible leading whitespaces
					match self.symbols.peek() {
						Some(symbol) if is_comment(**symbol) => (), // Is comment. Continue parsing
						_ => break, // Either none or not a comment. End parsing
					};
					comment.push('\n');
				}
				// Push next letter to comment
				Some(symbol) => {
					comment.push(**symbol);
					postion.position.end = symbol.position.position.end;
					self.symbols.next();
				},
				// File read to end
				None => break,
			}
		}
		// Remove potential trailing whitespaces
		comment = comment.trim().to_owned();
		PositionContainer::new(comment, postion)
	}
}


/// Parses a string to a keyword or to an identifier.
fn parse_string(string: PositionContainer<String>) -> ParseResult {
	Ok(match string.as_str() {
		"def" => Token::new(TokenKind::FunctionDefinition, string.position),
		"extern" => Token::new(TokenKind::Extern, string.position),
		"bitor" => Token::new(TokenKind::BitOr, string.position),
		"bitand" => Token::new(TokenKind::BitAnd, string.position),
		"mod" => Token::new(TokenKind::Modulus, string.position),
		"if" => Token::new(TokenKind::If, string.position),
		"else" => Token::new(TokenKind::Else, string.position),
		"for" => Token::new(TokenKind::For, string.position),
		_ => Token::new(TokenKind::Identifier(string.deref().to_owned()), string.position),
	})
}

/// Parses a number to a [TokenType::Number].
fn parse_number(number_str: PositionContainer<String>) -> ParseResult {
	// TODO: Add parsing for other number types.
	let number: f64 = match number_str.parse() {
		Ok(num) => num,
		Err(_) => return Err(Error::ParseNumberError(number_str))?,
	};
	Ok(Token::new(TokenKind::Number(number), number_str.position))
}

/// Checks whether `letter` is a letter that starts a comment line.
fn is_comment(letter: char) -> bool {
	letter == '#'
}

/// Checks whether `letter` is a special character like `+`, `-`, `=`, `*`.
fn is_special_char(letter: char) -> bool {
	[
		'+', '-', '=', '<', '*', '(', ')', '{', '}', '.', ':', ',', '/', ';', '[', ']',
	].contains(&letter)
}


impl<T> Iterator for Lexer<T> where
	T: Iterator<Item = Symbol>,
{
	type Item = ParseResult;

	fn next(&mut self) -> Option<Self::Item> {
		self.tokenize_next_item()
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lexer() {

	}
}