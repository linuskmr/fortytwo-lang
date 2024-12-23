use std::{ops::Deref, sync::Arc};

use crate::{
	ast::{expression::FunctionCall, statement::DataType},
	semantic_analyzer::variable::Variable,
	source::{PositionContainer, SourcePositionRange},
};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
	#[error("{}: Redeclaration: Variable `{new_declaration}` was previously declared as `{previous_declaration}`.", new_declaration.name.position)]
	Redeclaration { previous_declaration: Arc<Variable>, new_declaration: Arc<Variable> },

	#[error("{}: UndeclaredVariable: Variable `{name}` is not declared.", name.position)]
	UndeclaredVariable { name: PositionContainer<String> },

	#[error("{}: TypeMismatch: expected {}, got {}", position, expected, actual)]
	TypeMismatch { expected: DataType, position: SourcePositionRange, actual: DataType },

	#[error("{}: UndefinedFunctionCall: Call of function `{}(...)`, but no such function is defined.", function_call.name.position, function_call.name.deref())]
	UndefinedFunctionCall { function_call: FunctionCall },

	#[error("{}: ArgumentCountMismatch: Function `{}(...)` expects {expected} arguments but {actual} parameters provided", function_call.name.position, function_call.name.value)]
	ArgumentCountMismatch { expected: usize, actual: usize, function_call: FunctionCall },
}
