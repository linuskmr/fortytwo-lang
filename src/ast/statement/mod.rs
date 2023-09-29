mod basic_data_type;
mod data_type;
mod var_assignment;

pub use crate::ast::function_argument::FunctionArgument;
pub use crate::ast::function_definition::FunctionDefinition;
pub use crate::ast::function_prototype::FunctionPrototype;
pub use crate::ast::statement::var_assignment::{VariableAssignment, VariableDeclaration};
pub use basic_data_type::BasicDataType;
pub use data_type::DataType;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
	VariableDeclaration(VariableDeclaration),
	VariableAssignment(VariableAssignment),
}
