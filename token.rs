use crate::position_container::{PositionRange, PositionRangeContainer};
use crate::position_reader::Symbol;

/// A number indicating which precedence a token has over others.
pub type Precedence = u8;

pub type Token = PositionRangeContainer<TokenType>;

#[derive(Debug, Clone)]
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