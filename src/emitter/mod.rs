//! Generating a specific target code from AST nodes.

mod c;
mod ftl;

pub use c::Emitter as C;
pub use ftl::Emitter as Ftl;

/// Generates (target) code from AST nodes.
pub trait Emitter {
	/// Generate code from the AST nodes and write it to the `writer`.
	fn codegen(ast_nodes: impl Iterator<Item = crate::ast::Node>, writer: Box<dyn std::io::Write>) -> std::io::Result<()>;
}