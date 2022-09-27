use crate::ast::{Instruction, Expression, Block};
use crate::ast::function_prototype::FunctionPrototype;

/// A function definition.
#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
	/// The function prototype of this function, i.e. the header.
	pub prototype: FunctionPrototype,
	/// The body of the function.
	pub body: Block,
}