use crate::{
	ast::{statement::DataType, Expression},
	source::PositionContainer,
};

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
