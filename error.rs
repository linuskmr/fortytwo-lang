extern crate serde;
use serde::Serialize;
use crate::position_container::PositionRange;
use crate::position_reader::Symbol;

/// A error occurred while parsing the sourcecode.
#[derive(Debug, Clone, Serialize)]
pub struct ParsingError {
    /// The kind of this error.
    pub kind: ParsingErrorKind,
    /// An additional message.
    pub msg: String,
    /// The position this error occurred.
    pub position: PositionRange,
}

impl ParsingError {
    /// Creates a new ParsingError and takes over the position from the given [Symbol].
    pub(crate) fn from_symbol(symbol: &Symbol, kind: ParsingErrorKind, message: String) -> Self {
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
    fn kind(&self) -> ParsingErrorKind {
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

impl From<&ParsingError> for ParsingError {
    fn from(parsing_error: &ParsingError) -> Self {
        Self {
            kind: parsing_error.kind.clone(),
            msg: parsing_error.msg.clone(),
            position: parsing_error.position.clone()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ParsingErrorKind {
    ExpectedExpression,
    ExpectedSymbol,
    UnknownSymbol,
}