use crate::ast::expression::FunctionCall;
use crate::ast::statement::DataType;
use crate::semantic_analyzer::variable::Variable;
use crate::source::{PositionContainer, SourcePositionRange};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
	#[error("{}: Redeclaration: Variable `{new_declaration}` was previously declared as `{previous_declaration}`.", new_declaration.name.position)]
	Redeclaration {
		previous_declaration: Arc<Variable>,
		new_declaration: Arc<Variable>,
	},

	#[error("{}: UndeclaredVariable: Variable `{name}` is not declared.", name.position)]
	UndeclaredVariable { name: PositionContainer<String> },

	#[error("{}: TypeMismatch: expected {}, got {}", position, expected, actual)]
	TypeMismatch {
		expected: DataType,
		position: SourcePositionRange,
		actual: DataType,
	},

	#[error("{}: UndefinedFunctionCall: Call of function `{}(...)`, but no such function is defined.", function.name.position, function.name.deref())]
	UndefinedFunctionCall { function: FunctionCall },
}
