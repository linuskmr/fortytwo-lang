use std::{
	collections::{HashMap, HashSet},
	convert::Infallible,
	hash::{Hash, Hasher},
	iter, marker,
	ops::Deref,
	sync::Arc,
};

use crate::{
	ast,
	ast::{
		expression::{BinaryExpression, Number, NumberKind},
		statement::{BasicDataType, DataType},
		Expression, FunctionDefinition, FunctionPrototype, Struct,
	},
	source::{Position, PositionContainer},
};

#[derive(Debug)]
pub struct SymbolTable {
	/// All declared functions in the program, as discovered by the [global symbol scan](pass::GlobalSymbolScan).
	pub functions: HashMap<String, FunctionPrototype>,
	/// All declared structs in the program, as discovered by the [global symbol scan](pass::GlobalSymbolScan).
	pub structs: HashMap<String, Struct>,
}

impl Default for SymbolTable {
	fn default() -> Self {
		Self { functions: HashMap::new(), structs: HashMap::new() }
	}
}

impl SymbolTable {
	/// Scans the program for global symbols like [struct](crate::ast::struct_) and [function definitions](crate::ast::FunctionDefinition).
	///
	/// This is the first pass of the semantic analyzer, which is used to build the [structs](Self::structs) and [functions symbol tables](Self::functions).
	/// Afterwards, the [type check pass](Self::type_check) may be run.
	#[tracing::instrument(skip_all)]
	pub fn global_symbol_scan<'a>(ast_nodes: impl Iterator<Item = &'a ast::Node>) -> Result<Self, Infallible> {
		let mut symbol_table = SymbolTable::default();
		for ast_node in ast_nodes {
			symbol_table.ast_node(ast_node)?;
		}
		Ok(symbol_table)
	}

	/// Scans one AST node for global symbols, i.e. functions and structs.
	fn ast_node(&mut self, node: &ast::Node) -> Result<(), Infallible> {
		match node {
			ast::Node::Function(function) => self.function(&function.prototype),
			ast::Node::Struct(struct_) => self.struct_(struct_),
			ast::Node::FunctionPrototype(function_prototype) => self.function(function_prototype),
			_ => todo!(),
		}
	}

	/// Adds a function to the [functions symbol table](Self::functions).
	fn function(&mut self, function_prototype: &FunctionPrototype) -> Result<(), Infallible> {
		self.functions.insert(function_prototype.name.deref().clone(), function_prototype.clone());
		Ok(())
	}

	/// Adds a struct to the [structs symbol table](Self::structs).
	fn struct_(&mut self, struct_: &Struct) -> Result<(), Infallible> {
		self.structs.insert(struct_.name.deref().clone(), struct_.clone());
		Ok(())
	}
}
