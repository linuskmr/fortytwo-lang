use crate::position_container::{PositionRange, PositionRangeContainer};
use crate::position_reader::Symbol;

/// A number indicating which precedence a token has over others.
pub type Precedence = u8;

pub type Token = PositionRangeContainer<TokenType>;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn precedence(&self) -> Option<Precedence> {
        self.data.precedence()
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

    pub fn precedence(&self) -> Option<Precedence> {
        match self {
            TokenType::Less => Some(10),
            TokenType::Plus => Some(20),
            TokenType::Minus => Some(20),
            TokenType::Star => Some(40),
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

    #[test]
    fn precedence() {
        assert_eq!(TokenType::Less.precedence(), Some(10));
        assert_eq!(TokenType::Plus.precedence(), Some(20));
        assert_eq!(TokenType::Minus.precedence(), Some(20));
        assert_eq!(TokenType::Star.precedence(), Some(40));
    }
}