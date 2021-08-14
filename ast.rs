//! The Abstract Syntax Tree.

use crate::position_container::PositionRangeContainer;
use crate::token::{Token, TokenType};
use std::convert::TryFrom;
use crate::error::{FTLError, FTLErrorKind};
use std::cmp::Ordering;

/// A node of an Abstract Syntax Tree.
#[derive(Debug)]
pub enum AstNode {
    Expression(Expression),
    Statement(Statement),
}

#[derive(Debug)]
pub(crate) enum Expression {
    BinaryExpression(BinaryExpression),
    FunctionCall(FunctionCall),
    Number(PositionRangeContainer<f64>),
    Variable(PositionRangeContainer<String>),
}

#[derive(Debug)]
pub(crate) enum Statement {
    FunctionPrototype(FunctionPrototype),
    Function(Function),
}

/// A function argument consists of a name and a type that specify an argument of a function in its function prototype.
#[derive(Debug)]
pub(crate) struct FunctionArgument {
    /// The name of the function argument.
    pub(crate) name: PositionRangeContainer<String>,
    /// The type of the argument, e.g. float or string.
    pub(crate) typ: PositionRangeContainer<String>,
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
    /// A compare if the left is less than the right.
    Less,
    /// A addition (`+`).
    Addition,
    /// A subtraction (`-`).
    Subtraction,
    /// A multiplication (`*`)
    Multiplication,
}

/// A number indicating which precedence a token has over others. A higher precedence means that this
/// [BinaryOperator] is preferred over others with less [Precedence].
pub type Precedence = u8;

impl BinaryOperator {
    /*/// Creates a new binary operator from a [Token] or returns None, if no [BinaryOperator] can be found for the
    /// [Token].
    pub fn from_token(token: &Token) -> Option<BinaryOperator> {
        let t: &TokenType = &token.data;
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
    }*/

    /// Returns the precedence which a [BinaryOperator] has over others. A higher precedence means more precedence.
    pub fn precedence(&self) -> Precedence {
        match self {
            BinaryOperator::Less => 10,
            BinaryOperator::Addition => 20,
            BinaryOperator::Subtraction => 20,
            BinaryOperator::Multiplication => 40,
        }
    }
}

impl TryFrom<&Token> for BinaryOperator {
    type Error = FTLError;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match &token.data {
            TokenType::Less => Ok(BinaryOperator::Less),
            TokenType::Star => Ok(BinaryOperator::Multiplication),
            TokenType::Plus => Ok(BinaryOperator::Addition),
            TokenType::Minus => Ok(BinaryOperator::Subtraction),
            _ => Err(FTLError {
                kind: FTLErrorKind::IllegalToken,
                msg: format!("Expected binary operator token, got {:?}", token),
                position: token.position.clone(),
            }),
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
