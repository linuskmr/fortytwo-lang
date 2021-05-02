use std::io;
use std::path::Iter;

use ast::AST;
use lexer::{Lexer, Token, TokenType};

struct Parser<R: io::Read> {
    lexer: Lexer<R>,
    current_token: Option<Token>,
}

impl<R: io::Read> Parser<R> {
    /// Creates a new Parser with the lexer.
    fn new(lexer: Lexer<R>) -> Self {
        Self {
            lexer,
            current_token: None,
        }
    }

    /// Reads the next token from self.lexer, writes it into self.current_token and returns it.
    fn get_next_token(&mut self) -> &Option<Token> {
        self.current_token = self.lexer.next();
        &self.current_token
    }

    /// Parses a number to a ast::Number.
    ///
    /// # Panics
    ///
    /// Panics if self.current_token does not contain a TokenType::Number or is None.
    fn parse_number(&self) -> ast::Number {
        match self.current_token {
            Some(Token { token_type: TokenType::Number(number) }) => {
                ast::Number(number)
            }
            _ => panic!("parse_number called on an illegal token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let code = "abc +12.3 -4 def";
        let code = code.as_bytes();
        let l = Lexer::new(code);
    }
}