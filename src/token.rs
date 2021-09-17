use crate::position_container::PositionRangeContainer;

pub type Token = PositionRangeContainer<TokenKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Keyword: Function definition.
    FunctionDefinition,
    /// Function, variable name or data type.
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
    /// {
    OpeningCurlyBraces,
    /// }
    ClosingCurlyBraces,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// :
    Colon,
    /// /
    Slash,
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
    /// If
    If,
    /// Else
    Else,
    /// For
    For,
    /// .
    Dot,
    /// End of line, i.e. `\n`.
    EndOfLine,
}
