//! C emitter.

use crate::ast;
use crate::ast::expression::BinaryOperator;
use crate::ast::statement::{BasicDataType, DataType};
use crate::ast::Expression;
use crate::source::PositionContainer;
use std::fs::write;
use std::io;
use std::ops::Deref;

/// C emitter.
pub struct Emitter {
	writer: Box<dyn io::Write>,
}

impl Emitter {
	pub fn codegen(
		ast_nodes: impl Iterator<Item = ast::Node>,
		writer: Box<dyn io::Write>,
	) -> io::Result<()> {
		let mut this = Self { writer };

		// Prelude
		writeln!(this.writer, "#include <stdio.h>");

		for ast_node in ast_nodes {
			this.ast_node(ast_node)?;
		}
		Ok(())
	}

	fn ast_node(&mut self, node: ast::Node) -> io::Result<()> {
		match node {
			ast::Node::Function(function) => self.function(function),
			ast::Node::Struct(struct_) => self.struct_(struct_),
			ast::Node::FunctionPrototype(_) => Ok(()), // extern function
			_ => todo!(),
		}
	}

	fn function(&mut self, function: ast::FunctionDefinition) -> io::Result<()> {
		// Function header
		write!(self.writer, "void {}(", *function.prototype.name)?;
		for arg in function.prototype.args {
			self.function_argument(arg)?;
			write!(self.writer, ", ")?; // TODO: Remove trailing comma
		}
		writeln!(self.writer, ") {{")?;

		// Function body
		for instruction in function.body {
			self.instruction(instruction)?;
		}
		writeln!(self.writer)?;
		writeln!(self.writer, "}}")?;
		Ok(())
	}

	fn struct_(&mut self, struct_: ast::Struct) -> io::Result<()> {
		writeln!(self.writer, "typedef struct {{",)?;
		for field in struct_.fields {
			self.data_type(field.data_type)?;
			write!(self.writer, " {};", *field.name)?;
		}
		writeln!(self.writer, "}} {};", *struct_.name)?;
		Ok(())
	}

	fn instruction(&mut self, instruction: ast::Instruction) -> io::Result<()> {
		match instruction {
			ast::Instruction::Expression(expression) => self.expression(expression),
			ast::Instruction::Statement(statement) => self.statement(statement),
			ast::Instruction::IfElse(if_else) => self.if_else(*if_else),
			ast::Instruction::WhileLoop(while_loop) => self.while_loop(*while_loop),
		}
	}

	fn expression(&mut self, expression: ast::Expression) -> io::Result<()> {
		match expression {
			Expression::BinaryExpression(binary_expression) => {
				self.binary_expression(binary_expression)
			}
			Expression::FunctionCall(function_call) => self.function_call(function_call),
			Expression::Number(number) => self.number(number),
			Expression::Variable(variable) => self.variable(variable),
		}
	}

	fn binary_expression(
		&mut self,
		binary_expression: ast::expression::BinaryExpression,
	) -> io::Result<()> {
		self.expression(*binary_expression.lhs)?;
		let operator = match *binary_expression.operator {
			ast::expression::BinaryOperator::Add => "+",
			ast::expression::BinaryOperator::Subtract => "-",
			ast::expression::BinaryOperator::Multiply => "*",
			ast::expression::BinaryOperator::Divide => "/",
			BinaryOperator::Less => "<",
			BinaryOperator::Greater => ">",
			BinaryOperator::Equal => "==",
			BinaryOperator::NotEqual => "=/=",
		};
		write!(self.writer, " {} ", operator)?;
		self.expression(*binary_expression.rhs)?;
		Ok(())
	}

	fn function_call(&mut self, function_call: ast::expression::FunctionCall) -> io::Result<()> {
		write!(self.writer, "{}(", *function_call.name)?;
		for param in function_call.params {
			self.expression(param)?;
		}
		writeln!(self.writer, ");")?;
		Ok(())
	}

	fn statement(&mut self, statement: ast::Statement) -> io::Result<()> {
		match statement {
			ast::statement::Statement::VariableDeclaration(variable_declaration) => {
				self.variable_declaration(variable_declaration)
			}
			ast::statement::Statement::VariableAssignment(assignment) => {
				self.variable_assignment(assignment)
			}
		}
	}

	fn variable_declaration(
		&mut self,
		variable_declaration: ast::statement::VariableDeclaration,
	) -> io::Result<()> {
		write!(
			self.writer,
			"{} {} = ",
			*variable_declaration.data_type, *variable_declaration.name
		)?;
		self.expression(variable_declaration.value)?;
		writeln!(self.writer, ";")?;
		Ok(())
	}

	fn variable_assignment(
		&mut self,
		assignment: ast::statement::VariableAssignment,
	) -> io::Result<()> {
		write!(self.writer, "{} = ", *assignment.name)?;
		self.expression(assignment.value)?;
		writeln!(self.writer, ";")?;
		Ok(())
	}

	fn if_else(&mut self, if_else: ast::IfElse) -> io::Result<()> {
		// if block, always present
		write!(self.writer, "if (")?;
		self.expression(if_else.condition)?;
		writeln!(self.writer, ") {{")?;
		for instruction in if_else.if_true {
			self.instruction(instruction)?;
		}
		writeln!(self.writer, "}}")?;

		// else block, optional
		if if_else.if_false.is_empty() {
			return Ok(());
		}
		writeln!(self.writer, "else {{")?;
		for instruction in if_else.if_false {
			self.instruction(instruction)?;
		}
		writeln!(self.writer, "}}")?;

		Ok(())
	}

	fn while_loop(&mut self, while_loop: ast::WhileLoop) -> io::Result<()> {
		write!(self.writer, "while (")?;
		self.expression(while_loop.condition)?;
		writeln!(self.writer, ") {{")?;
		for instruction in while_loop.body {
			self.instruction(instruction)?;
		}
		writeln!(self.writer, "}}")?;
		Ok(())
	}

	fn function_argument(
		&mut self,
		function_argument: ast::statement::FunctionArgument,
	) -> io::Result<()> {
		write!(self.writer, "{}: ", *function_argument.name)?;
		self.data_type(function_argument.data_type)?;
		Ok(())
	}

	fn data_type(
		&mut self,
		data_type: PositionContainer<ast::statement::DataType>,
	) -> io::Result<()> {
		match data_type.inner {
			DataType::Basic(basic_data_type) => self.basic_data_type(basic_data_type),
			DataType::Struct(struct_name) => self.struct_name(struct_name),
			DataType::Pointer(pointer) => self.pointer(pointer),
		}
	}

	fn basic_data_type(
		&mut self,
		basic_data_type: ast::statement::BasicDataType,
	) -> io::Result<()> {
		match basic_data_type {
			BasicDataType::Int => write!(self.writer, "int"),
			BasicDataType::Float => write!(self.writer, "float"),
		}
	}

	fn struct_name(&mut self, struct_name: String) -> io::Result<()> {
		write!(self.writer, "{}", struct_name)
	}

	fn pointer(
		&mut self,
		pointer: Box<PositionContainer<ast::statement::DataType>>,
	) -> io::Result<()> {
		write!(self.writer, "ptr")?;
		self.data_type(*pointer)
	}

	fn number(&mut self, number: ast::expression::Number) -> io::Result<()> {
		match *number {
			ast::expression::NumberKind::Int(int) => write!(self.writer, "{}", int)?,
			ast::expression::NumberKind::Float(float) => write!(self.writer, "{}", float)?,
		}
		Ok(())
	}

	fn variable(&mut self, variable: ast::expression::Variable) -> io::Result<()> {
		write!(self.writer, "{}", *variable)?;
		Ok(())
	}
}
