//! Semantic analysis includes type checking and creating the symbol table.

mod error;
mod expression_type_inference;
pub mod pass;
mod variable;

use crate::ast;
use crate::ast::statement::{BasicDataType, DataType};
use crate::ast::{FunctionDefinition, FunctionPrototype, Struct};
use crate::semantic_analyzer::expression_type_inference::expression_type_inference;
use crate::source::PositionContainer;
pub use error::Error;
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
	pub functions: HashMap<Name, FunctionPrototype>,
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
			ast::Node::Function(function) => self.function(&function.prototype),
			ast::Node::Struct(struct_) => self.struct_(struct_),
			ast::Node::FunctionPrototype(function_prototype) => self.function(function_prototype),
			_ => todo!(),
		}
	}

	fn function(&mut self, function_prototype: &FunctionPrototype) -> Result<(), Infallible> {
		self.functions.insert(
			function_prototype.name.deref().clone(),
			function_prototype.clone(),
		);
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
			ast::Node::FunctionPrototype(_) => Ok(()),
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
		if let None = self.functions.get(&function_call.name.deref().clone()) {
			return Err(Error::UndefinedFunctionCall {
				function: function_call.clone(),
			});
		}
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
		let var = Arc::new(Variable {
			name: variable_declaration.name.clone(),
			type_: variable_declaration.data_type.deref().clone(),
		});
		tracing::debug!(
			var = var.to_string(),
			position = var.name.position.to_string(),
			"variable declaration"
		);

		if expression_type_inference(&variable_declaration.value)? != var.type_ {
			return Err(Error::TypeMismatch {
				expected: var.type_.clone(),
				position: variable_declaration.name.position.clone(),
				actual: expression_type_inference(&variable_declaration.value)?,
			});
		}

		// If there is a previous declaration of this variable, this is always a problem.
		let previous_declaration = self.variables.get(&var);
		if let Some(previous_declaration) = previous_declaration {
			return Err(Error::Redeclaration {
				previous_declaration: Arc::clone(previous_declaration),
				new_declaration: Arc::clone(&var),
			});
		}

		self.add_variable(var)?;
		self.expression(&variable_declaration.value)?;
		Ok(())
	}

	/// Adds a variable to [`Self::variables`] and [`Self::call_stack`].
	fn add_variable(&mut self, var: Arc<Variable>) -> Result<(), Error> {
		self.variables.insert(Arc::clone(&var));
		self.call_stack.last_mut().unwrap().insert(var);
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
		let assignment_type = expression_type_inference(&variable_assignment.value)?;
		let var = Arc::new(Variable {
			name: variable_assignment.name.clone(),
			type_: assignment_type.clone(),
		});
		tracing::debug!(
			var = var.to_string(),
			position = var.name.position.to_string(),
			"variable assignment"
		);

		let previous_declaration = self.variables.get(&var).ok_or(Error::UndeclaredVariable {
			name: var.name.clone(),
		})?;

		if assignment_type != previous_declaration.type_ {
			return Err(Error::TypeMismatch {
				expected: previous_declaration.type_.clone(),
				position: variable_assignment.name.position.clone(),
				actual: assignment_type.clone(),
			});
		}

		self.add_variable(var)?;
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
