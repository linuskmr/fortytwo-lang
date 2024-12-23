use std::fmt::Display;

use super::Expression;
use crate::ast::Block;

/// Execute the `body` *while* the `condition` is true.
#[derive(Debug, PartialEq, Clone)]
pub struct WhileLoop {
	pub condition: Expression,
	pub body: Block,
}
