use std::fmt::Debug;
use lexer::{SpecialCharacter, PositionRangeContainer};

/// An Abstract Syntax Tree.
pub enum AST {
    /// A binary expression of the form `lhs op rhs`.
    BinaryExpression {
        /// The left hand side.
        lhs: Box<AST>,
        /// The operator connecting [lhs] and [rhs].
        operator: PositionRangeContainer<SpecialCharacter>,
        /// The right hand side.
        rhs: Box<AST>,
    },
    FunctionPrototype(FunctionPrototype),
    Function {
        prototype: FunctionPrototype,
        body: Box<AST>,
    },
    FunctionCall {
        name: PositionRangeContainer<String>,
        args: Vec<Box<AST>>,
    },
    Number(PositionRangeContainer<f64>),
    Variable(PositionRangeContainer<String>),
}

pub struct FunctionPrototype {
    pub name: PositionRangeContainer<String>,
    pub args: Vec<PositionRangeContainer<String>>,
}