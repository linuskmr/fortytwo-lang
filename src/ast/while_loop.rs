use super::Expression;
use crate::ast::Block;
use std::fmt::Display;

/// Execute the `body` *while* the `condition` is true.
#[derive(Debug, PartialEq, Clone)]
pub struct WhileLoop {
	pub condition: Expression,
	pub body: Block,
}
