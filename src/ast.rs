//! The Abstract Syntax Tree.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::position_container::PositionRangeContainer;
use crate::token::TokenKind;

/// A node of an Abstract Syntax Tree. Either an expression or a statement.
#[derive(Debug, PartialEq)]
pub enum AstNode {
    Expression(Expression),
    Statement(Statement),
}

/// Binary expression, function call, number or variable.
#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    FunctionCall(FunctionCall),
    Number(PositionRangeContainer<f64>),
    Variable(PositionRangeContainer<String>),
    IfExpression(Box<IfExpression>),
}

/// Function or function prototype.
#[derive(Debug, PartialEq)]
pub enum Statement {
    FunctionPrototype(FunctionPrototype),
    Function(Function),
}

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
#[derive(Debug, PartialEq)]
pub struct IfExpression {
    pub(crate) condition: Expression,
    pub(crate) if_true: Expression,
    pub(crate) if_false: Expression,
}

/// A function argument consists of a name and a type that specify an argument of a function in its function prototype.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct FunctionArgument {
    /// The name of the function argument.
    pub(crate) name: PositionRangeContainer<String>,
    /// The type of the argument, e.g. a int, a struct or a pointer.
    pub(crate) data_type: PositionRangeContainer<DataType>,
}

/// A data type is either basic, a struct, or a pointer to a data type.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum DataType {
    /// A basic data type like int and float.
    Basic(BasicDataType),
    /// A user defined struct with custom name.
    Struct(String),
    /// A Pointer to a data type.
    Pointer(Box<PositionRangeContainer<DataType>>),
}

/// A basic data type is a type with hardware support like int and float.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BasicDataType {
    /// A integer number, like 42
    Int,
    /// A floating point number like 4.2
    Float,
}

impl TryFrom<&str> for BasicDataType {
    type Error = ();

    /// Converts a data type (as string) to a [BasicDataType] enum. If `data_type` does not match any [BasicDataType],
    /// this method will return Err.
    fn try_from(data_type: &str) -> Result<Self, Self::Error> {
        match data_type {
            "int" => Ok(BasicDataType::Int),
            "float" => Ok(BasicDataType::Float),
            _ => Err(()), // No basic data type with this name
        }
    }
}

/// A function call, i.e. the execution of a [Function] with concrete parameters.
#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    /// The name of the function to be called.
    pub name: PositionRangeContainer<String>,
    /// The parameters to invoke the called function with.
    pub params: Vec<Expression>,
}

/// A function definition.
#[derive(Debug, PartialEq)]
pub struct Function {
    /// The function prototype of this function, i.e. the header.
    pub prototype: FunctionPrototype,
    /// The body of the function.
    pub body: Expression,
}

/// A binary expression of the form `lhs op rhs` like `40 + 2`.
#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    /// The left hand side.
    pub lhs: Box<Expression>,
    /// The operator connecting `lhs` and `rhs`.
    pub operator: PositionRangeContainer<BinaryOperator>,
    /// The right hand side.
    pub rhs: Box<Expression>,
}

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
}

impl PartialOrd for BinaryOperator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Precedence is a number indicating which precedence a token has over others. A higher precedence means that
        // this BinaryOperator is preferred over others with less precedence.
        // TODO: Use a lazy_static HashMap here
        let mut precedence = HashMap::new();
        precedence.insert(BinaryOperator::Less, 10);
        precedence.insert(BinaryOperator::Greater, 10);
        precedence.insert(BinaryOperator::Add, 20);
        precedence.insert(BinaryOperator::Subtract, 20);
        precedence.insert(BinaryOperator::Multiply, 30);
        precedence.insert(BinaryOperator::Divide, 30);

        precedence[self].partial_cmp(&precedence[other])
    }
}

impl TryFrom<TokenKind> for BinaryOperator {
    type Error = ();

    fn try_from(token_kind: TokenKind) -> Result<Self, Self::Error> {
        match token_kind {
            TokenKind::Less => Ok(BinaryOperator::Less),
            TokenKind::Star => Ok(BinaryOperator::Multiply),
            TokenKind::Plus => Ok(BinaryOperator::Add),
            TokenKind::Minus => Ok(BinaryOperator::Subtract),
            TokenKind::Slash => Ok(BinaryOperator::Divide),
            _other => Err(()),
        }
    }
}

/// A function prototype, i.e. the header of the function. It consists of the function name and arguments.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct FunctionPrototype {
    /// The name of the function.
    pub name: PositionRangeContainer<String>,
    /// The arguments for the function.
    pub args: Vec<FunctionArgument>,
}
