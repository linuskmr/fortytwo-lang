use crate::ast;
use crate::source::PositionContainer;
use std::{io, todo};

/// *Visitor pattern* for visiting each [`Node`] of an AST.
///
/// This is used by the [semantic analyzer](crate::semantic_analyzer) and [code emitters](crate::emitter).
pub trait Visitor {
	type Err: std::error::Error;

	fn ast_node(&mut self, node: ast::Node) -> Result<(), Self::Err> {
		match node {
			ast::Node::Function(function) => self.function(function),
			ast::Node::Struct(struct_) => self.struct_(struct_),
			_ => todo!(),
		}
	}

	fn function(&mut self, function: ast::FunctionDefinition) -> Result<(), Self::Err> {
		// Function header
		for arg in function.prototype.args {
			self.function_argument(arg)?;´
		}

		// Function body
		for instruction in function.body {
			self.instruction(instruction)?;
		}
		Ok(())
	}

	fn struct_(&mut self, struct_: ast::Struct) -> Result<(), Self::Err> {
		for field in struct_.fields {
			self.data_type(field.data_type)?;
		}
		Ok(())
	}

	fn instruction(&mut self, instruction: ast::Instruction) -> Result<(), Self::Err> {
		match instruction {
			ast::Instruction::Expression(expression) => self.expression(expression),
			ast::Instruction::Statement(statement) => self.statement(statement),
			ast::Instruction::IfElse(if_else) => self.if_else(*if_else),
			ast::Instruction::WhileLoop(while_loop) => self.while_loop(*while_loop),
		}
	}

	fn expression(&mut self, expression: ast::Expression) -> Result<(), Self::Err> {
		match expression {
			ast::Expression::BinaryExpression(binary_expression) => {
				self.binary_expression(binary_expression)
			}
			ast::Expression::FunctionCall(function_call) => self.function_call(function_call),
			ast::Expression::Number(number) => self.number(number),
			ast::Expression::Variable(variable) => self.variable(variable),
		}
	}

	fn binary_expression(
		&mut self,
		binary_expression: ast::expression::BinaryExpression,
	) -> Result<(), Self::Err> {
		self.expression(*binary_expression.lhs)?;
		self.expression(*binary_expression.rhs)?;
		Ok(())
	}

	fn function_call(
		&mut self,
		function_call: ast::expression::FunctionCall,
	) -> Result<(), Self::Err> {
		for param in function_call.params {
			self.expression(param)?;
		}
		Ok(())
	}

	fn statement(&mut self, statement: ast::Statement) -> Result<(), Self::Err> {
		match statement {
			ast::statement::Statement::VariableDeclaration(variable_declaration) => {
				self.variable_declaration(variable_declaration)
			}
			ast::statement::Statement::VariableAssignment(assignment) => {
				self.assignment(assignment)
			}
		}
	}

	fn variable_declaration(
		&mut self,
		variable_declaration: ast::statement::VariableDeclaration,
	) -> Result<(), Self::Err> {
		self.expression(variable_declaration.value)?;
		Ok(())
	}

	fn assignment(
		&mut self,
		assignment: ast::statement::VariableAssignment,
	) -> Result<(), Self::Err> {
		self.expression(assignment.value)?;
		Ok(())
	}

	fn if_else(&mut self, if_else: ast::IfElse) -> Result<(), Self::Err> {
		// if block, always present
		self.expression(if_else.condition)?;
		for instruction in if_else.if_true {
			self.instruction(instruction)?;
		}

		// else block, optional
		if if_else.if_false.is_empty() {
			return Ok(());
		}
		for instruction in if_else.if_false {
			self.instruction(instruction)?;
		}

		Ok(())
	}

	fn while_loop(&mut self, while_loop: ast::WhileLoop) -> Result<(), Self::Err> {
		self.expression(while_loop.condition)?;
		for instruction in while_loop.body {
			self.instruction(instruction)?;
		}
		Ok(())
	}

	fn function_argument(
		&mut self,
		function_argument: ast::statement::FunctionArgument,
	) -> Result<(), Self::Err> {
		self.data_type(function_argument.data_type)?;
		Ok(())
	}

	fn data_type(
		&mut self,
		data_type: PositionContainer<ast::statement::DataType>,
	) -> Result<(), Self::Err> {
		match data_type.inner {
			ast::statement::DataType::Basic(basic_data_type) => {
				self.basic_data_type(basic_data_type)
			}
			ast::statement::DataType::Struct(struct_name) => self.struct_name(struct_name),
			ast::statement::DataType::Pointer(pointer) => self.pointer(pointer),
		}
	}

	fn basic_data_type(
		&mut self,
		basic_data_type: ast::statement::BasicDataType,
	) -> Result<(), Self::Err> {
		Ok(())
	}

	fn struct_name(&mut self, struct_name: String) -> Result<(), Self::Err> {
		Ok(())
	}

	fn pointer(
		&mut self,
		pointer: Box<PositionContainer<ast::statement::DataType>>,
	) -> Result<(), Self::Err> {
		self.data_type(*pointer)
	}

	fn number(&mut self, number: ast::expression::Number) -> Result<(), Self::Err> {
		Ok(())
	}

	fn variable(&mut self, variable: ast::expression::Variable) -> Result<(), Self::Err> {
		Ok(())
	}
}
