//! Semantic analysis includes type checking and creating the symbol table.

use crate::ast::FunctionDefinition;
use std::collections::HashSet;

pub struct SemanticAnalyzer {
	pub functions: Vec<FunctionDefinition>,
	/// Currently declared in-scope variables.
	pub variables: HashSet<Variable>,
}

pub struct Variable {
	pub name: String,
	pub value: Option<VariableValue>,
}

pub enum VariableValue {
	F64(f64),
	Str(String),
	Struct(()),
}
