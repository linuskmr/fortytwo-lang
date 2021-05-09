use std::fmt::Debug;

use crate::position_container::PositionRangeContainer;
use crate::token::Token;

/// An Abstract Syntax Tree.
pub enum AST {
    /// A binary expression of the form `lhs op rhs`.
    BinaryExpression {
        /// The left hand side.
        lhs: Box<AST>,
        /// The operator connecting `lhs` and `rhs`.
        operator: PositionRangeContainer<Token>,
        /// The right hand side.
        rhs: Box<AST>,
    },
    /// A function prototype.
    FunctionPrototype(FunctionPrototype),
    /// A function definition.
    Function {
        prototype: FunctionPrototype,
        /// The body of the function.
        body: Box<AST>,
    },
    FunctionCall {
        /// The name of the called function.
        name: PositionRangeContainer<String>,
        /// The arguments for the called function.
        args: Vec<Box<AST>>,
    },
    Number(PositionRangeContainer<f64>),
    Variable(PositionRangeContainer<String>),
}

pub struct FunctionPrototype {
    /// The name of the function.
    pub name: PositionRangeContainer<String>,
    /// The arguments of this function.
    pub args: Vec<PositionRangeContainer<String>>,
}
