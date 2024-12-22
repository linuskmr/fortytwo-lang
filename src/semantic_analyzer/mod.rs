//! Semantic analysis includes type checking and creating the symbol table.

mod error;
pub mod pass;
mod variable;

use crate::ast;
use crate::ast::expression::{BinaryExpression, Number, NumberKind};
use crate::ast::statement::{BasicDataType, DataType};
use crate::ast::Expression;
use crate::ast::{FunctionDefinition, FunctionPrototype, Struct};
use crate::source::Position;
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

/// Stores all variables declared in this call stack frame.
type CallStackFrame = HashSet<Arc<Variable>>;

#[derive(Debug)]
pub struct SemanticAnalyzer<Pass> {
	/// All declared functions in the program, as discovered by the [global symbol scan](pass::GlobalSymbolScan).
	pub functions: HashMap<Name, FunctionPrototype>,
	/// All declared structs in the program, as discovered by the [global symbol scan](pass::GlobalSymbolScan).
	pub structs: HashMap<Name, Struct>,
	/// Currently declared in-scope variables.
	pub variables: HashMap<String, Arc<Variable>>,
	/// List of stack frames, each containing the variables declared in that scope.
	pub call_stack: Vec<CallStackFrame>,
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
			variables: HashMap::new(),
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
		self.call_stack.push(CallStackFrame::new());

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
			ast::Expression::Number(number) => Ok(()),
			ast::Expression::Variable(variable) => Ok(()),
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
		if !self
			.functions
			.contains_key(&function_call.name.deref().clone())
		{
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

	/// Checks that the type of the expression matches that of the variable.
	fn variable_declaration(
		&mut self,
		variable_declaration: &ast::statement::VariableDeclaration,
	) -> Result<(), Error> {
		let variable = Arc::new(Variable {
			name: variable_declaration.name.clone(),
			type_: variable_declaration.data_type.deref().clone(),
		});
		tracing::debug!(
			var = variable.to_string(),
			position = variable.name.position.to_string(),
			"variable declaration"
		);

		let inferred_type = self.infer_expression_type(&variable_declaration.value)?;
		if inferred_type != variable.type_ {
			return Err(Error::TypeMismatch {
				expected: variable.type_.clone(),
				position: variable.name.position.clone(),
				actual: inferred_type,
			});
		}

		// If there is a previous declaration of this variable, there is a name conflict.
		let previous_declaration = self.variables.get(&variable.name.inner);
		if let Some(previous_declaration) = previous_declaration {
			return Err(Error::Redeclaration {
				previous_declaration: Arc::clone(previous_declaration),
				new_declaration: Arc::clone(&variable),
			});
		}

		self.add_variable(variable)?;
		// Type check the expression itself
		// TODO: Should already be covered by the type inference of the expression, i.e. by calling `self.infer_expression_type`
		self.expression(&variable_declaration.value)?;
		Ok(())
	}

	/// Adds a variable to [`Self::variables`] and [`Self::call_stack`].
	fn add_variable(&mut self, var: Arc<Variable>) -> Result<(), Error> {
		self.variables
			.insert(var.name.inner.clone(), Arc::clone(&var));
		self.call_stack.last_mut().unwrap().insert(var);
		Ok(())
	}

	/// Removes one frame from the call stack and deletes all of its variables from the symbol table ([`Self::variables`]).
	fn drop_call_stack_frame(&mut self) {
		let frame = self.call_stack.pop().unwrap();
		for variable in frame {
			self.variables.remove(&variable.name.inner);
		}
	}

	/// Checks that the type of the expression matches that of the variable.
	fn variable_assignment(
		&mut self,
		variable_assignment: &ast::statement::VariableAssignment,
	) -> Result<(), Error> {
		// Infer the type of the expression on the right-hand side of the assignment
		let expression_type = self.infer_expression_type(&variable_assignment.value)?;
		let var = Arc::new(Variable {
			name: variable_assignment.name.clone(),
			type_: expression_type.clone(),
		});
		tracing::debug!(
			var = var.to_string(),
			position = var.name.position.to_string(),
			"variable assignment"
		);

		// Look up the type of the variable in the symbol table
		let variable_type =
			self.variables
				.get(&var.name.inner)
				.ok_or(Error::UndeclaredVariable {
					name: var.name.clone(),
				})?;

		if expression_type != variable_type.type_ {
			// Cannot assign an expression to a variable of different type
			return Err(Error::TypeMismatch {
				expected: variable_type.type_.clone(),
				position: variable_assignment.name.position.clone(),
				actual: expression_type.clone(),
			});
		}

		self.add_variable(var)?;
		self.expression(&variable_assignment.value)?;
		Ok(())
	}

	/// Type checks an if-else block.
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

	/// Type checks a while loop.
	fn while_loop(&mut self, while_loop: &ast::WhileLoop) -> Result<(), Error> {
		self.expression(&while_loop.condition)?;
		for instruction in &while_loop.body {
			self.instruction(instruction)?;
		}
		Ok(())
	}

	/// Infers the type of an expression, which can consist of binary expressions, numbers, function calls and variables.
	pub fn infer_expression_type(&self, expression: &Expression) -> Result<DataType, Error> {
		match expression {
			Expression::BinaryExpression(binary_expression) => {
				self.infer_binary_expression_type(binary_expression)
			}
			Expression::FunctionCall(function_call) => todo!("Function call type inference"),
			Expression::Number(number) => Self::number_type_inference(number),
			Expression::Variable(variable) => {
				// Here, a variables is used inside an expression. This is not about a variable declaration.
				self.infer_variable_type(variable)
			}
		}
	}

	/// Infers the type of the left-hand and right-hand side of a binary expression,
	/// verifies that they are equal and returns this common type.
	fn infer_binary_expression_type(
		&self,
		binary_expression: &BinaryExpression,
	) -> Result<DataType, Error> {
		let lhs = self.infer_expression_type(&binary_expression.lhs)?;
		let rhs = self.infer_expression_type(&binary_expression.rhs)?;
		if lhs != rhs {
			return Err(Error::TypeMismatch {
				expected: lhs,
				position: binary_expression.operator.position.clone(),
				actual: rhs,
			});
		}
		Ok(lhs)
	}

	/// Infers the type of a variable by looking it up in [`Self::variables`].
	fn infer_variable_type(&self, variable: &PositionContainer<String>) -> Result<DataType, Error> {
		self.variables
			.get(&variable.inner)
			.map(|v| v.type_.clone())
			.ok_or(Error::UndeclaredVariable {
				name: variable.clone(),
			})
	}

	fn number_type_inference(number: &Number) -> Result<DataType, Error> {
		match number.inner {
			NumberKind::Int(_) => Ok(DataType::Basic(BasicDataType::Int)),
			NumberKind::Float(_) => Ok(DataType::Basic(BasicDataType::Float)),
		}
	}
}
