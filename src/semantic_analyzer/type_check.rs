use std::{
	collections::{HashMap, HashSet},
	convert::Infallible,
	hash::{Hash, Hasher},
	iter, marker,
	ops::Deref,
	sync::Arc,
};

use super::{symbol_table, Error, SymbolTable, Variable};
use crate::{
	ast,
	ast::{
		expression::{BinaryExpression, Number, NumberKind},
		statement::{BasicDataType, DataType},
		Expression, FunctionDefinition, FunctionPrototype, Struct,
	},
	source::{Position, PositionContainer},
};

/// Stores all variables declared in this call stack frame.
type CallStackFrame = HashSet<Arc<Variable>>;

pub struct TypeChecker {
	symbol_table: SymbolTable,
	/// Currently declared in-scope variables.
	pub variables: HashMap<String, Arc<Variable>>,
	/// List of stack frames, each containing the variables declared in that scope.
	pub call_stack: Vec<CallStackFrame>,
}

impl TypeChecker {
	/// Verifies that all types in the program match the expected types.
	#[tracing::instrument(skip_all)]
	pub fn type_check<'a>(
		symbol_table: SymbolTable,
		ast_nodes: impl Iterator<Item = &'a ast::Node>,
	) -> Result<(), Error> {
		let mut type_check = Self { symbol_table, variables: HashMap::new(), call_stack: Vec::new() };

		type_check.call_stack.push(CallStackFrame::new());

		for ast_node in ast_nodes {
			type_check.ast_node(ast_node)?;
		}
		Ok(())
	}

	/// Type checks an AST node by calling the appropriate method for the node type.
	fn ast_node(&mut self, node: &ast::Node) -> Result<(), Error> {
		match node {
			ast::Node::Function(function) => self.function(function),
			ast::Node::Struct(struct_) => Ok(()),
			ast::Node::FunctionPrototype(_) => Ok(()),
			_ => todo!(),
		}
	}

	/// Type checks each instruction in the given function.
	#[tracing::instrument(skip_all, fields(name = function.prototype.name.deref()))]
	fn function(&mut self, function: &FunctionDefinition) -> Result<(), Error> {
		for instruction in &function.body {
			self.instruction(instruction)?;
		}
		Ok(())
	}

	/// Type checks an instruction by calling the appropriate method for the instruction type.
	fn instruction(&mut self, instruction: &ast::Instruction) -> Result<(), Error> {
		match instruction {
			ast::Instruction::Expression(expression) => self.expression(expression),
			ast::Instruction::Statement(statement) => self.statement(statement),
			ast::Instruction::IfElse(if_else) => self.if_else(if_else),
			ast::Instruction::WhileLoop(while_loop) => self.while_loop(while_loop),
		}
	}

	/// Type checks an expression by calling the appropriate method for the expression type.
	fn expression(&mut self, expression: &ast::Expression) -> Result<(), Error> {
		match expression {
			ast::Expression::BinaryExpression(binary_expression) => self.binary_expression(binary_expression),
			ast::Expression::FunctionCall(function_call) => self.function_call(function_call),
			ast::Expression::Number(number) => Ok(()),
			ast::Expression::Variable(variable) => Ok(()),
		}
	}

	/// Type checks a binary expression.
	fn binary_expression(&mut self, binary_expression: &ast::expression::BinaryExpression) -> Result<(), Error> {
		self.expression(&binary_expression.lhs)?;
		self.expression(&binary_expression.rhs)?;
		Ok(())
	}

	/// Checks that the called function exists and that supplied parameter types match the defined argument types.
	fn function_call(&mut self, function_call: &ast::expression::FunctionCall) -> Result<(), Error> {
		let function_definition = self.symbol_table.functions.get(&function_call.name.inner);
		let Some(function_definition) = function_definition else {
			return Err(Error::UndefinedFunctionCall { function_call: function_call.clone() });
		};

		// Since the later used `iter::zip` returns None if one of the iterators is shorter than the other, we need to check the lengths first.
		if function_call.params.len() != function_definition.args.len() {
			return Err(Error::ArgumentCountMismatch {
				expected: function_definition.args.len(),
				actual: function_call.params.len(),
				function_call: function_call.clone(),
			});
		}

		for (param, arg) in iter::zip(&function_call.params, &function_definition.args) {
			let param_type = self.infer_expression_type(param)?;
			if param_type != arg.data_type.inner {
				return Err(Error::TypeMismatch {
					expected: arg.data_type.inner.clone(),
					position: param.source_position(),
					actual: param_type,
				});
			}
		}

		Ok(())
	}

	/// Type checks a statement.
	fn statement(&mut self, statement: &ast::Statement) -> Result<(), Error> {
		match statement {
			ast::statement::Statement::VariableDeclaration(variable_declaration) => {
				self.variable_declaration(variable_declaration)
			},
			ast::statement::Statement::VariableAssignment(assignment) => self.variable_assignment(assignment),
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
		self.variables.insert(var.name.inner.clone(), Arc::clone(&var));
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
	fn variable_assignment(&mut self, variable_assignment: &ast::statement::VariableAssignment) -> Result<(), Error> {
		// Infer the type of the expression on the right-hand side of the assignment
		let expression_type = self.infer_expression_type(&variable_assignment.value)?;
		let var = Arc::new(Variable { name: variable_assignment.name.clone(), type_: expression_type.clone() });
		tracing::debug!(var = var.to_string(), position = var.name.position.to_string(), "variable assignment");

		// Look up the type of the variable in the symbol table
		let variable_type =
			self.variables.get(&var.name.inner).ok_or(Error::UndeclaredVariable { name: var.name.clone() })?;

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
			Expression::BinaryExpression(binary_expression) => self.infer_binary_expression_type(binary_expression),
			Expression::FunctionCall(function_call) => todo!("Function call type inference"),
			Expression::Number(number) => Self::number_type_inference(number),
			Expression::Variable(variable) => {
				// Here, a variables is used inside an expression. This is not about a variable declaration.
				self.infer_variable_type(variable)
			},
		}
	}

	/// Infers the type of the left-hand and right-hand side of a binary expression,
	/// verifies that they are equal and returns this common type.
	fn infer_binary_expression_type(&self, binary_expression: &BinaryExpression) -> Result<DataType, Error> {
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
			.ok_or(Error::UndeclaredVariable { name: variable.clone() })
	}

	fn number_type_inference(number: &Number) -> Result<DataType, Error> {
		match number.inner {
			NumberKind::Int(_) => Ok(DataType::Basic(BasicDataType::Int)),
			NumberKind::Float(_) => Ok(DataType::Basic(BasicDataType::Float)),
		}
	}
}
