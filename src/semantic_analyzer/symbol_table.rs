use std::{
	collections::HashMap,
	convert::Infallible,
	ops::Deref,
};

use crate::{
	ast,
	ast::{
		FunctionPrototype, Struct,
	},
};

/// Contains all globally declared [functions](Self::functions) and [structs](Self::structs).
#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
	/// All declared functions in the program, as discovered by the [global symbol scan](Self::global_symbol_scan).
	pub functions: HashMap<String, FunctionPrototype>,
	/// All declared structs in the program, as discovered by the [global symbol scan](Self::global_symbol_scan).
	pub structs: HashMap<String, Struct>,
}

impl SymbolTable {
	/// Generates a [`SymbolTable`] by scanning the program for global symbols like [struct](crate::ast::struct_) and [function definitions](crate::ast::FunctionDefinition).
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
