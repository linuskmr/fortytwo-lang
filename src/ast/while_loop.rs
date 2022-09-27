use crate::ast::Block;
use super::Expression;


#[derive(Debug, PartialEq, Clone)]
pub struct WhileLoop {
	pub condition: Expression,
	pub body: Block,
}