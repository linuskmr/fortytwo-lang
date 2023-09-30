use crate::semantic_analyzer::variable::Variable;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
	#[error("Variable `{new_declaration}` was already declared as `{previous_declaration}`.")]
	TypeCheckError {
		previous_declaration: Arc<Variable>,
		new_declaration: Arc<Variable>,
	},
}
