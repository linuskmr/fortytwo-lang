extern crate serde;
use serde::Serialize;
use crate::position_container::{PositionRange, PositionRangeContainer};
use crate::position_reader::Symbol;


pub type Token = PositionRangeContainer<TokenType>;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TokenType {
    /// Keyword: Function definition.
    Def,
    /// Function or variable name.
    Identifier(String),
    /// Keyword: Extern keyword.
    Extern,
    /// Data type: Floating point number.
    Number(f64),
    /// +
    Plus,
    /// *
    Star,
    /// -
    Minus,
    /// <
    Less,
    /// (
    OpeningParentheses,
    /// )
    ClosingParentheses,
    /// ,
    Comma,
    /// ;
    Semicolon,
}

impl Token {
    pub fn from_symbol(symbol: Symbol) -> Option<Self> {
        let token_type = TokenType::new(symbol.data)?;
        Some(Self {
            data: token_type,
            position: PositionRange {
                line: symbol.position.line,
                column: symbol.position.column.clone()..=symbol.position.column,
            },
        })
    }
}

impl TokenType {
    pub fn new(c: char) -> Option<Self> {
        match c {
            '(' => Some(TokenType::OpeningParentheses),
            ')' => Some(TokenType::ClosingParentheses),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            '<' => Some(TokenType::Less),
            '*' => Some(TokenType::Star),
            ';' => Some(TokenType::Semicolon),
            ',' => Some(TokenType::Comma),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_type() {
        assert_eq!(TokenType::new('('), Some(TokenType::OpeningParentheses));
        assert_eq!(TokenType::new(')'), Some(TokenType::ClosingParentheses));
        assert_eq!(TokenType::new('-'), Some(TokenType::Minus));
        assert_eq!(TokenType::new('+'), Some(TokenType::Plus));
        assert_eq!(TokenType::new('<'), Some(TokenType::Less));
        assert_eq!(TokenType::new('*'), Some(TokenType::Star));
        assert_eq!(TokenType::new(';'), Some(TokenType::Semicolon));
        assert_eq!(TokenType::new(','), Some(TokenType::Comma));
        assert_eq!(TokenType::new('0'), None);
    }
}