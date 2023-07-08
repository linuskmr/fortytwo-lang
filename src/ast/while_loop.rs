use super::Expression;
use crate::ast::Block;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Block,
}
