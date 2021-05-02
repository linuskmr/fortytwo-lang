use std::fmt::Debug;

pub use binary_operation::BinaryOperation;
pub use function::{Function, FunctionPrototype};
pub use function_call::FunctionCall;
pub use number::Number;
pub use variable::Variable;

mod number;
mod variable;
mod binary_operation;
mod function_call;
mod function;

pub trait AST: Debug {
    fn pretty(&self) -> String;
}