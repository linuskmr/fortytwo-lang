//! The Abstract Syntax Tree.

use crate::position_container::PositionRangeContainer;
use crate::token::{Token, TokenType};

/// A node of an Abstract Syntax Tree.
#[derive(Debug)]
pub enum AstNode {
    BinaryExpression(BinaryExpression),
    FunctionPrototype(FunctionPrototype),
    Function(Function),
    FunctionCall(FunctionCall),
    Number(PositionRangeContainer<f64>),
    Variable(PositionRangeContainer<String>),
}

/// A function call, i.e. the execution of a [Function] with concrete arguments.
#[derive(Debug)]
pub struct FunctionCall {
    /// The name of the called function.
    pub name: PositionRangeContainer<String>,
    /// The arguments for the called function.
    pub args: Vec<Box<AstNode>>,
}

/// A function definition.
#[derive(Debug)]
pub struct Function {
    /// The function prototype of this function, i.e. the header.
    pub prototype: FunctionPrototype,
    /// The body of the function.
    pub body: Box<AstNode>,
}

/// A binary expression of the form `lhs op rhs`.
#[derive(Debug)]
pub struct BinaryExpression {
    /// The left hand side.
    pub lhs: Box<AstNode>,
    /// The operator connecting `lhs` and `rhs`.
    pub operator: BinaryOperator,
    /// The right hand side.
    pub rhs: Box<AstNode>,
}

/// A binary operator connecting a lhs and a rhs.
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    /// A addition (`+`).
    Addition,
    /// A subtraction (`-`).
    Subtraction,
    /// A multiplication (`*`)
    Multiplication,
    /// A compare if the left is less than the right.
    Less,
}

/// A number indicating which precedence a token has over others. A higher precedence means that this
/// [BinaryOperator] is preferred over others with less [Precedence].
pub type Precedence = u8;

impl BinaryOperator {
    /// Creates a new binary operator from a [Token] or returns None, if no [BinaryOperator] can be found for the
    /// [Token].
    pub fn from_token(token: &Token) -> Option<BinaryOperator> {
        Self::from_token_type(&token.data)
    }

    /// Creates a new binary operator from a [TokenType] or returns [None], if no [BinaryOperator] can be found for the
    /// [Token].
    pub fn from_token_type(token_type: &TokenType) -> Option<BinaryOperator> {
        match token_type {
            TokenType::Less => Some(BinaryOperator::Less),
            TokenType::Star => Some(BinaryOperator::Multiplication),
            TokenType::Plus => Some(BinaryOperator::Addition),
            TokenType::Minus => Some(BinaryOperator::Subtraction),
            _ => None,
        }
    }

    /// Returns the precedence which a [BinaryOperator] has over others. A higher precedence means
    pub fn precedence(&self) -> Option<Precedence> {
        match self {
            BinaryOperator::Less => Some(10),
            BinaryOperator::Addition => Some(20),
            BinaryOperator::Subtraction => Some(20),
            BinaryOperator::Multiplication => Some(40),
            _ => None,
        }
    }
}

/// A function prototype, i.e. its header.
#[derive(Debug)]
pub struct FunctionPrototype {
    /// The name of the function.
    pub name: PositionRangeContainer<String>,
    /// The arguments for the function.
    pub args: Vec<PositionRangeContainer<String>>,
}
