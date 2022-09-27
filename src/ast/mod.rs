pub mod statement;
pub mod expression;
mod function_argument;
mod function_definition;
mod function_prototype;
pub mod struct_;
mod if_else;
mod while_loop;

pub use expression::Expression;
pub use statement::Statement;
pub use function_definition::FunctionDefinition;
pub use function_prototype::FunctionPrototype;
pub use struct_::Struct;
pub use if_else::IfElse;
pub use while_loop::WhileLoop;


/// A "regular" line of code.
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
	Expression(Expression),
	Statement(Statement),
	IfElse(Box<IfElse>),
	WhileLoop(Box<WhileLoop>),
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
	FunctionPrototype(FunctionPrototype),
	Function(FunctionDefinition),
	Struct(Struct),
}

pub type Block = Vec<Instruction>;
