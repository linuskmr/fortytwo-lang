use crate::position_container::PositionRange;
use crate::position_reader::Symbol;
use std::fmt;

/// A error occurred while parsing the sourcecode.
#[derive(Debug, Clone)]
pub struct FTLError {
    /// The kind of this error.
    pub kind: FTLErrorKind,
    /// An additional message.
    pub msg: String,
    /// The position this error occurred.
    pub position: PositionRange,
}

impl FTLError {
    /// Creates a new ParsingError and takes over the position from the given [Symbol].
    pub(crate) fn from_symbol(symbol: &Symbol, kind: FTLErrorKind, message: String) -> Self {
        Self {
            kind,
            msg: message,
            position: PositionRange {
                line: symbol.position.line,
                column: symbol.position.column..=symbol.position.column,
            },
        }
    }

    /// Returns the kind of this error.
    #[allow(unused)]
    fn kind(&self) -> FTLErrorKind {
        self.kind.clone()
    }

    /// Returns the message of this error.
    #[allow(unused)]
    fn message(&self) -> &str {
        &self.msg
    }

    /// Returns the position this error occurred.
    #[allow(unused)]
    fn position(&self) -> &PositionRange {
        &self.position
    }
}

impl fmt::Display for FTLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {:?}\n{}\nIn {}\n", self.kind, self.msg, self.position)
    }
}

impl From<&FTLError> for FTLError {
    fn from(parsing_error: &FTLError) -> Self {
        Self {
            kind: parsing_error.kind.clone(),
            msg: parsing_error.msg.clone(),
            position: parsing_error.position.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub enum FTLErrorKind {
    ExpectedExpression,
    IllegalSymbol,
    IllegalToken,
}