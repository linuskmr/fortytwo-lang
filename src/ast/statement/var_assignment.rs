use crate::ast::statement::DataType;
use crate::ast::Expression;
use crate::source::PositionContainer;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub name: PositionContainer<String>,
    pub data_type: PositionContainer<DataType>,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableAssignment {
    pub name: PositionContainer<String>,
    pub value: Expression,
}
