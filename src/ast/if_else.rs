use crate::ast::Block;
use super::Expression;

/// An if expression, like
/// ```text
/// if answer == 42 {
///     42
/// } else {
///     0
/// }
/// ```
/// * The `condition` is `answer == 42`.
/// * The `if_true` expression is `42`.
/// * The `if_false` expression is `0´.
#[derive(Debug, PartialEq, Clone)]
pub struct IfElse {
	pub condition: Expression,
	pub if_true: Block,
	pub if_false: Block,
}