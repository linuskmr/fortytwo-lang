use crate::PositionRange;
use crate::position_reader::Symbol;

pub struct UnknownSymbolError {
    pub msg: String,
    pub position: PositionRange,
}

impl UnknownSymbolError {
    pub fn from_symbol(symbol: &Symbol) -> Self {
        Self {
            msg: format!("Unknown symbol `{}`", symbol.data),
            position: PositionRange {
                line: symbol.position.line,
                column: symbol.position.column..=symbol.position.column
            }
        }
    }
}