
use crate::ast::{function_prototype::FunctionPrototype, Block};

/// Name, arguments and body define a function.
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition {
	/// Name and arguments of the function.
	pub prototype: FunctionPrototype,
	/// The body of the function.
	pub body: Block,
}
