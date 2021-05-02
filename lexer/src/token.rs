#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    /// Keyword: Function definition.
    Def,
    /// Function or variable name.
    Identifier(String),
    /// Keyword: Extern keyword.
    Extern,
    /// Data type: Floating point number.
    Number(f64),
    Other(char),
}