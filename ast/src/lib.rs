use std::fmt::Debug;

pub use binary_expression::BinaryExpression;
pub use function::{Function, FunctionPrototype};
pub use function_call::FunctionCall;
pub use number::Number;
pub use variable::Variable;

mod number;
mod variable;
mod binary_expression;
mod function_call;
mod function;

pub trait AST: Debug {
    fn pretty(&self) -> String;
}