//! Semantic analysis includes type checking and creating the symbol table.

mod error;
pub mod pass;
mod variable;

use crate::ast;
use crate::ast::statement::{BasicDataType, DataType};
use crate::ast::{FunctionDefinition, Struct};
use crate::semantic_analyzer::error::Error;
use crate::source::PositionContainer;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::hash::{Hash, Hasher};
use std::marker;
use std::ops::Deref;
use std::sync::Arc;
use variable::Variable;

type Name = String;

type Variables = HashSet<Arc<Variable>>;

#[derive(Debug)]
pub struct SemanticAnalyzer<Pass> {
	pub functions: HashMap<Name, FunctionDefinition>,
	pub structs: HashMap<Name, Struct>,
	/// Currently declared in-scope variables.
	pub variables: Variables,
	pub call_stack: Vec<Variables>,
	/// The current pass.
	///
	/// The pass isn't needed at runtime, but just for the compiler to differentiate between the implementations. Therefore, `PhantomData` is used here.
	pass: marker::PhantomData<Pass>,
}

impl Default for SemanticAnalyzer<pass::GlobalSymbolScan> {
	fn default() -> Self {
		Self {
			functions: HashMap::new(),
			structs: HashMap::new(),
			variables: HashSet::new(),
			call_stack: Vec::new(),
			pass: marker::PhantomData,
		}
	}
}

impl SemanticAnalyzer<pass::GlobalSymbolScan> {
	#[tracing::instrument(skip_all)]
	pub fn global_symbol_scan<'a>(
		mut self,
		ast_nodes: impl Iterator<Item = &'a ast::Node>,
	) -> Result<SemanticAnalyzer<pass::TypeCheck>, Infallible> {
		for ast_node in ast_nodes {
			self.ast_node(ast_node)?;
		}
		Ok(SemanticAnalyzer {
			functions: self.functions,
			structs: self.structs,
			variables: self.variables,
			call_stack: self.call_stack,
			pass: marker::PhantomData,
		})
	}

	fn ast_node(&mut self, node: &ast::Node) -> Result<(), Infallible> {
		match node {
			ast::Node::Function(function) => self.function(function),
			ast::Node::Struct(struct_) => self.struct_(struct_),
			_ => todo!(),
		}
	}

	fn function(&mut self, function: &FunctionDefinition) -> Result<(), Infallible> {
		self.functions
			.insert(function.prototype.name.deref().clone(), function.clone());
		Ok(())
	}

	fn struct_(&mut self, struct_: &Struct) -> Result<(), Infallible> {
		self.structs
			.insert(struct_.name.deref().clone(), struct_.clone());
		Ok(())
	}
}

impl SemanticAnalyzer<pass::TypeCheck> {
	#[tracing::instrument(skip_all)]
	pub fn type_check<'a>(
		mut self,
		ast_nodes: impl Iterator<Item = &'a ast::Node>,
	) -> Result<(), Error> {
		self.call_stack.push(Variables::new());

		for ast_node in ast_nodes {
			self.ast_node(ast_node)?;
		}
		Ok(())
	}

	fn ast_node(&mut self, node: &ast::Node) -> Result<(), Error> {
		match node {
			ast::Node::Function(function) => self.function(function),
			ast::Node::Struct(struct_) => Ok(()),
			_ => todo!(),
		}
	}

	#[tracing::instrument(skip_all, fields(name = function.prototype.name.deref()))]
	fn function(&mut self, function: &FunctionDefinition) -> Result<(), Error> {
		for instruction in &function.body {
			self.instruction(instruction)?;
		}
		Ok(())
	}

	fn instruction(&mut self, instruction: &ast::Instruction) -> Result<(), Error> {
		match instruction {
			ast::Instruction::Expression(expression) => self.expression(expression),
			ast::Instruction::Statement(statement) => self.statement(statement),
			ast::Instruction::IfElse(if_else) => self.if_else(if_else),
			ast::Instruction::WhileLoop(while_loop) => self.while_loop(while_loop),
		}
	}

	fn expression(&mut self, expression: &ast::Expression) -> Result<(), Error> {
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
		binary_expression: &ast::expression::BinaryExpression,
	) -> Result<(), Error> {
		self.expression(&binary_expression.lhs)?;
		self.expression(&binary_expression.rhs)?;
		Ok(())
	}

	fn function_call(
		&mut self,
		function_call: &ast::expression::FunctionCall,
	) -> Result<(), Error> {
		for param in &function_call.params {
			self.expression(param)?;
		}
		Ok(())
	}

	fn statement(&mut self, statement: &ast::Statement) -> Result<(), Error> {
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
		variable_declaration: &ast::statement::VariableDeclaration,
	) -> Result<(), Error> {
		let var = Variable {
			name: variable_declaration.name.deref().clone(),
			type_: variable_declaration.data_type.deref().clone(),
		};
		tracing::debug!(
			var = ?var,
			"variable declaration",
		);
		self.add_variable(var)?;
		self.expression(&variable_declaration.value)?;
		Ok(())
	}

	/// Adds a variable to [`Self::variables`] and [`Self::call_stack`].
	fn add_variable(&mut self, var: Variable) -> Result<(), Error> {
		let var = Arc::new(var);
		let previous_declaration = self.variables.get(&var);
		if let Some(previous_declaration) = previous_declaration {
			return Err(Error::TypeCheckError {
				previous_declaration: Arc::clone(previous_declaration),
				new_declaration: Arc::clone(&var),
			});
		}
		self.variables.insert(Arc::clone(&var));
		self.call_stack.last_mut().unwrap().insert(Arc::clone(&var));
		Ok(())
	}

	/// Remove all variables from the current scope from [`Self::variables`].
	fn drop_call_stack_frame(&mut self) {
		let frame = self.call_stack.pop().unwrap();
		self.variables.retain(|v| !frame.contains(v));
	}

	fn variable_assignment(
		&mut self,
		variable_assignment: &ast::statement::VariableAssignment,
	) -> Result<(), Error> {
		let var = Variable {
			name: variable_assignment.name.deref().clone(),
			type_: DataType::Basic(BasicDataType::Int),
		};
		tracing::debug!(
			var = ?var,
			"variable assignment",
		);
		self.add_variable(var);
		self.expression(&variable_assignment.value)?;
		Ok(())
	}

	fn if_else(&mut self, if_else: &ast::IfElse) -> Result<(), Error> {
		// if block, always present
		self.expression(&if_else.condition)?;
		for instruction in &if_else.if_true {
			self.instruction(instruction)?;
		}

		// else block, optional
		if if_else.if_false.is_empty() {
			return Ok(());
		}
		for instruction in &if_else.if_false {
			self.instruction(instruction)?;
		}

		Ok(())
	}

	fn while_loop(&mut self, while_loop: &ast::WhileLoop) -> Result<(), Error> {
		self.expression(&while_loop.condition)?;
		for instruction in &while_loop.body {
			self.instruction(instruction)?;
		}
		Ok(())
	}

	fn function_argument(
		&mut self,
		function_argument: &ast::statement::FunctionArgument,
	) -> Result<(), Error> {
		self.data_type(&function_argument.data_type)?;
		Ok(())
	}

	fn data_type(
		&mut self,
		data_type: &PositionContainer<ast::statement::DataType>,
	) -> Result<(), Error> {
		match &data_type.inner {
			ast::statement::DataType::Basic(basic_data_type) => {
				self.basic_data_type(basic_data_type)
			}
			ast::statement::DataType::Struct(struct_name) => self.struct_name(struct_name),
			ast::statement::DataType::Pointer(pointer) => self.pointer(pointer),
		}
	}

	fn basic_data_type(
		&mut self,
		basic_data_type: &ast::statement::BasicDataType,
	) -> Result<(), Error> {
		Ok(())
	}

	fn struct_name(&mut self, struct_name: &str) -> Result<(), Error> {
		Ok(())
	}

	fn pointer(
		&mut self,
		pointer: &Box<PositionContainer<ast::statement::DataType>>,
	) -> Result<(), Error> {
		self.data_type(pointer)
	}

	fn number(&mut self, number: &ast::expression::Number) -> Result<(), Error> {
		Ok(())
	}

	fn variable(&mut self, variable: &ast::expression::Variable) -> Result<(), Error> {
		Ok(())
	}
}
