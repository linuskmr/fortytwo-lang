use crate::ast::function_prototype::FunctionPrototype;
use crate::ast::{Block, Expression, Instruction};
use std::fmt::Display;

/// A function definition.
#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    /// The function prototype of this function, i.e. the header.
    pub prototype: FunctionPrototype,
    /// The body of the function.
    pub body: Block,
}
