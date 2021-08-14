use crate::position_container::{PositionRangeContainer};

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
    /// :
    Colon,
    /// End of line, i.e. `\n`.
    EndOfLine
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