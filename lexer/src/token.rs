use crate::position_container::PositionRangeContainer;
use phf::{phf_map};

/// A number indicating which precedence a token has over others.
pub type Precedence = u8;

pub type Token = PositionRangeContainer<TokenType>;

impl Token {
    pub fn fun() -> i32 {0}
}

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
    Other(SpecialCharacter),
}

#[derive(Debug, Clone)]
pub struct SpecialCharacter(pub char);

impl SpecialCharacter {
    pub fn precedence(&self) -> Option<Precedence> {
        BINARY_OPERATION_PRECEDENCE.get(&self.0).cloned()
    }
}

static BINARY_OPERATION_PRECEDENCE: phf::Map<char, Precedence> = phf_map! {
    '<' => 10,
    '+' => 20,
    '-' => 20,
    '*' => 40,
};