use crate::ast;
use crate::ast::Node;
use crate::source::PositionContainer;
use std::{io, todo};

/// *Visitor pattern* for visiting each [`Node`] of an AST.
///
/// This is used by the [semantic analyzer](crate::semantic_analyzer) and [code emitters](crate::emitter).
pub trait Visitor {
	type Err: std::error::Error;

	fn ast_node(&mut self, node: Node) -> Result<(), Self::Err>;
	fn function(&mut self, function: ast::FunctionDefinition) -> Result<(), Self::Err>;
	fn struct_(&mut self, struct_: ast::Struct) -> Result<(), Self::Err>;
	fn function_argument(
		&mut self,
		function_argument: ast::statement::FunctionArgument,
	) -> Result<(), Self::Err>;
	fn data_type(
		&mut self,
		data_type: PositionContainer<ast::statement::DataType>,
	) -> Result<(), Self::Err>;
	fn basic_data_type(
		&mut self,
		basic_data_type: ast::statement::BasicDataType,
	) -> Result<(), Self::Err>;
	fn struct_name(&mut self, struct_name: String) -> Result<(), Self::Err>;
	fn pointer(
		&mut self,
		pointer: Box<PositionContainer<ast::statement::DataType>>,
	) -> Result<(), Self::Err>;
	fn instruction(&mut self, instruction: ast::Instruction) -> Result<(), Self::Err>;
	fn expression(&mut self, expression: ast::Expression) -> Result<(), Self::Err>;
	fn binary_expression(
		&mut self,
		binary_expression: ast::expression::BinaryExpression,
	) -> Result<(), Self::Err>;
	fn function_call(
		&mut self,
		function_call: ast::expression::FunctionCall,
	) -> Result<(), Self::Err>;
	fn number(&mut self, number: ast::expression::Number) -> Result<(), Self::Err>;
	fn variable(&mut self, variable: ast::expression::Variable) -> Result<(), Self::Err>;
	fn statement(&mut self, statement: ast::Statement) -> Result<(), Self::Err>;
	fn variable_declaration(
		&mut self,
		variable_declaration: ast::statement::VariableDeclaration,
	) -> Result<(), Self::Err>;
	fn assignment(
		&mut self,
		assignment: ast::statement::VariableAssignment,
	) -> Result<(), Self::Err>;
	fn if_else(&mut self, if_else: ast::IfElse) -> Result<(), Self::Err>;
	fn while_loop(&mut self, while_loop: ast::WhileLoop) -> Result<(), Self::Err>;
}
