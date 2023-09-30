//! Abstract syntax tree.

pub mod expression;
mod function_argument;
mod function_definition;
mod function_prototype;
mod if_else;
pub mod statement;
pub mod struct_;
mod visitor;
mod while_loop;

pub use expression::Expression;
pub use function_definition::FunctionDefinition;
pub use function_prototype::FunctionPrototype;
pub use if_else::IfElse;
pub use statement::Statement;
use std::fmt::Display;
pub use struct_::Struct;
pub use visitor::Visitor;
pub use while_loop::WhileLoop;

/// A "regular" line of code.
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
	Expression(Expression),
	Statement(Statement),
	IfElse(Box<IfElse>),
	WhileLoop(Box<WhileLoop>),
}

/// The top-level element of an AST.
#[derive(Debug, PartialEq)]
pub enum Node {
	FunctionPrototype(FunctionPrototype),
	Function(FunctionDefinition),
	Struct(Struct),
}

/// A list of instructions.
pub type Block = Vec<Instruction>;
