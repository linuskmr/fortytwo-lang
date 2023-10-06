use crate::ast::expression::{BinaryExpression, Number, NumberKind};
use crate::ast::statement::{BasicDataType, DataType};
use crate::ast::Expression;
use crate::semantic_analyzer::error::Error;
use crate::source::Position;

pub fn expression_type_inference(expression: &Expression) -> Result<DataType, Error> {
	match expression {
		Expression::BinaryExpression(binary_expression) => {
			binary_expression_type_inference(binary_expression)
		}
		Expression::FunctionCall(function_call) => todo!("Function call type inference"),
		Expression::Number(number) => number_type_inference(number),
		Expression::Variable(variable) => todo!("Variable type inference"),
	}
}

fn binary_expression_type_inference(
	binary_expression: &BinaryExpression,
) -> Result<DataType, Error> {
	let lhs = expression_type_inference(&binary_expression.lhs)?;
	let rhs = expression_type_inference(&binary_expression.rhs)?;
	if lhs != rhs {
		return Err(Error::TypeMismatch {
			expected: lhs,
			position: binary_expression.operator.position.clone(),
			actual: rhs,
		});
	}
	Ok(lhs)
}

fn number_type_inference(number: &Number) -> Result<DataType, Error> {
	match number.inner {
		NumberKind::Int(_) => Ok(DataType::Basic(BasicDataType::Int)),
		NumberKind::Float(_) => Ok(DataType::Basic(BasicDataType::Float)),
	}
}
