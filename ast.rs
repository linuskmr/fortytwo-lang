use crate::position_container::PositionRangeContainer;
use crate::token::{Token, TokenType};

/// An Abstract Syntax Tree.
#[derive(Debug)]
pub enum AST {
    /// A binary expression of the form `lhs op rhs`.
    BinaryExpression(BinaryExpression),
    /// A function prototype.
    FunctionPrototype(FunctionPrototype),
    /// A function definition.
    Function(Function),
    FunctionCall(FunctionCall),
    Number(PositionRangeContainer<f64>),
    Variable(PositionRangeContainer<String>),
}

#[derive(Debug)]
pub struct FunctionCall {
    /// The name of the called function.
    pub name: PositionRangeContainer<String>,
    /// The arguments for the called function.
    pub args: Vec<Box<AST>>,
}

/// A function definition.
#[derive(Debug)]
pub struct Function {
    pub prototype: FunctionPrototype,
    /// The body of the function.
    pub body: Box<AST>,
}

/// A binary expression of the form `lhs op rhs`.
#[derive(Debug)]
pub struct BinaryExpression {
    /// The left hand side.
    pub lhs: Box<AST>,
    /// The operator connecting `lhs` and `rhs`.
    pub operator: BinaryOperator,
    /// The right hand side.
    pub rhs: Box<AST>,
}

/// A binary operator connecting a lhs and a rhs.
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Times,
    /// <
    Less,
}

/// A number indicating which precedence a token has over others.
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
            TokenType::Star => Some(BinaryOperator::Times),
            TokenType::Plus => Some(BinaryOperator::Plus),
            TokenType::Minus => Some(BinaryOperator::Minus),
            _ => None,
        }
    }

    pub fn precedence(&self) -> Option<Precedence> {
        match self {
            BinaryOperator::Less => Some(10),
            BinaryOperator::Plus => Some(20),
            BinaryOperator::Minus => Some(20),
            BinaryOperator::Times => Some(40),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct FunctionPrototype {
    /// The name of the function.
    pub name: PositionRangeContainer<String>,
    /// The arguments of this function.
    pub args: Vec<PositionRangeContainer<String>>,
}
