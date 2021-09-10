//! The Abstract Syntax Tree.

use crate::position_container::PositionRangeContainer;
use crate::token::{Token, TokenType};
use std::convert::TryFrom;


/// A node of an Abstract Syntax Tree. Either an expression or a statement.
#[derive(Debug)]
pub(crate) enum AstNode {
    Expression(Expression),
    Statement(Statement),
}

/// Binary expression, function call, number or variable.
#[derive(Debug)]
pub(crate) enum Expression {
    BinaryExpression(BinaryExpression),
    FunctionCall(FunctionCall),
    Number(Number),
    Variable(Variable),
}

pub(crate) type Number = PositionRangeContainer<f64>;
pub(crate) type Variable = PositionRangeContainer<String>;

/// Function or function prototype.
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
    /// The type of the argument, e.g. a int, a struct or a pointer.
    pub(crate) kind: DataType,
}

/// A data type is either basic, a struct, or a pointer to a data type.
#[derive(Debug)]
pub(crate) enum DataTypeKind {
    /// A basic data type like int and float.
    Basic(BasicDataTypeKind),
    /// A user defined struct with custom name.
    Struct(String),
    /// A Pointer to a data type.
    Pointer(Box<DataTypeKind>)
}

pub(crate) type DataType = PositionRangeContainer<DataTypeKind>;

/// A basic data type is a type with hardware support like int and float.
#[derive(Debug)]
pub enum BasicDataTypeKind {
    /// A integer number, like 42
    Int,
    /// A floating point number like 4.2
    Float
}

impl TryFrom<&str> for BasicDataTypeKind {
    type Error = ();

    /// Converts a data type (as string) to a [BasicDataType] enum. If `data_type` does not match any [BasicDataType],
    /// this method will return Err.
    fn try_from(data_type: &str) -> Result<Self, Self::Error> {
        match data_type {
            "int" => Ok(BasicDataTypeKind::Int),
            "float" => Ok(BasicDataTypeKind::Float),
            _ => Err(()) // No basic data type with this name
        }
    }
}

/// A function call, i.e. the execution of a [Function] with concrete arguments.
#[derive(Debug)]
pub(crate) struct FunctionCall {
    /// The name of the called function.
    pub name: PositionRangeContainer<String>,
    /// The arguments for the called function.
    pub args: Vec<FunctionArgument>,
}

/// A function definition.
#[derive(Debug)]
pub(crate) struct Function {
    /// The function prototype of this function, i.e. the header.
    pub prototype: FunctionPrototype,
    /// The body of the function.
    pub body: BinaryExpression,
}

/// A binary expression of the form `lhs op rhs`.
#[derive(Debug)]
pub(crate) struct BinaryExpression {
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
    type Error = ();

    /// Tries to convert a token into a BinaryOperator. On failure returns and empty Err.
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match &token.data {
            TokenType::Less => Ok(BinaryOperator::Less),
            TokenType::Star => Ok(BinaryOperator::Multiplication),
            TokenType::Plus => Ok(BinaryOperator::Addition),
            TokenType::Minus => Ok(BinaryOperator::Subtraction),
            _ => Err(()),
        }
    }
}

/// A function prototype, i.e. its header.
#[derive(Debug)]
pub(crate) struct FunctionPrototype {
    /// The name of the function.
    pub name: PositionRangeContainer<String>,
    /// The arguments for the function.
    pub args: Vec<FunctionArgument>,
}
