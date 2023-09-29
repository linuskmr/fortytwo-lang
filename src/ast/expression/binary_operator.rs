use crate::token::TokenKind;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

// TODO: Implement Copy for BinaryOperator? See parser::Parser::parse_binary_operation_rhs() at `If the next binary
//  operator binds stronger with rhs than with current, let it go with rhs`
/// A binary operator connecting a lhs and a rhs.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BinaryOperator {
	/// Comparison if lhs is smaller/less than rhs (`<`).
	Less,
	/// Comparison if lhs is bigger/greater than rhs (`>`).
	Greater,
	/// Addition (`+`).
	Add,
	/// Subtraction (`-`).
	Subtract,
	/// Multiplication (`*`)
	Multiply,
	/// Division (`/`)
	Divide,
	Equal,
	NotEqual,
}

impl PartialOrd for BinaryOperator {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		// Precedence is a number indicating which precedence a token has over others. A higher precedence means that
		// this BinaryOperator is preferred over others with less precedence.
		// TODO: Use a 'lazy_static HashMap' or 'phf map' here
		let mut precedence = HashMap::new();
		precedence.insert(BinaryOperator::Less, 10);
		precedence.insert(BinaryOperator::Greater, 10);
		precedence.insert(BinaryOperator::Add, 20);
		precedence.insert(BinaryOperator::Subtract, 20);
		precedence.insert(BinaryOperator::Multiply, 30);
		precedence.insert(BinaryOperator::Divide, 30);
		precedence.insert(BinaryOperator::Equal, 5);
		precedence.insert(BinaryOperator::NotEqual, 5);

		precedence[self].partial_cmp(&precedence[other])
	}
}
