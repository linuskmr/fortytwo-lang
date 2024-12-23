//! Creation of a [`SymbolTable`] and [type checking](TypeChecker).

mod error;
mod symbol_table;
mod type_check;
mod variable;

pub use error::Error;
pub use symbol_table::SymbolTable;
pub use type_check::TypeChecker;
pub use variable::Variable;
