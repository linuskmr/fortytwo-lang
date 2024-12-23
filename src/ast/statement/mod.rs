mod basic_data_type;
mod data_type;
mod var_assignment;


pub use basic_data_type::BasicDataType;
pub use data_type::DataType;

pub use crate::ast::{
	function_argument::FunctionArgument,
	function_definition::FunctionDefinition,
	function_prototype::FunctionPrototype,
	statement::var_assignment::{VariableAssignment, VariableDeclaration},
};

use super::Expression;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
	VariableDeclaration(VariableDeclaration),
	VariableAssignment(VariableAssignment),
	Return(Expression),
}
