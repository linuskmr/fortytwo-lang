use crate::position_container::PositionRangeContainer;

pub type Token = PositionRangeContainer<TokenKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Keyword: Function definition.
    Def,
    /// Function or variable name.
    Identifier(String),
    /// Keyword: Extern keyword.
    Extern,
    /// Data type: Floating point number.
    Number(f64),
    /// Comment (Possible a doc comment)
    Comment(String),
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
    /// =
    Equal,
    /// =/=
    NotEqual,
    /// Bitwise OR
    BitOr,
    /// Bitwise AND
    BitAnd,
    /// Modulus %
    Modulus,
    /// .
    Dot,
    /// End of line, i.e. `\n`.
    EndOfLine,
}
