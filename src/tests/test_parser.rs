
use crate::ast::DataType::Basic;
use crate::ast::{
    AstNode, BasicDataType, BinaryExpression, BinaryOperator, DataType, Expression, Function,
    FunctionArgument, FunctionPrototype, Statement,
};
use crate::error::FTLError;
use crate::parser;
use crate::position_container::{PositionRange, PositionRangeContainer};

#[test]
fn parse_extern() {
    let parser =
        parser::sourcecode_to_parser("extern write(fd: int, buf: ptr int, len: int)".chars());
    let expected: [Result<AstNode, FTLError>; 1] = [Ok(AstNode::Statement(
        Statement::FunctionPrototype(FunctionPrototype {
            name: PositionRangeContainer {
                data: String::from("write"),
                position: PositionRange {
                    line: 1,
                    column: 8..=12,
                },
            },
            args: vec![
                FunctionArgument {
                    name: PositionRangeContainer {
                        data: String::from("fd"),
                        position: PositionRange {
                            line: 1,
                            column: 14..=15,
                        },
                    },
                    data_type: PositionRangeContainer {
                        data: DataType::Basic(BasicDataType::Int),
                        position: PositionRange {
                            line: 1,
                            column: 18..=20,
                        },
                    },
                },
                FunctionArgument {
                    name: PositionRangeContainer {
                        data: String::from("buf"),
                        position: PositionRange {
                            line: 1,
                            column: 23..=25,
                        },
                    },
                    data_type: PositionRangeContainer {
                        data: DataType::Pointer(Box::new(PositionRangeContainer {
                            data: Basic(BasicDataType::Int),
                            position: PositionRange {
                                line: 1,
                                column: 32..=34,
                            },
                        })),
                        position: PositionRange {
                            line: 1,
                            column: 28..=34,
                        },
                    },
                },
                FunctionArgument {
                    name: PositionRangeContainer {
                        data: String::from("len"),
                        position: PositionRange {
                            line: 1,
                            column: 37..=39,
                        },
                    },
                    data_type: PositionRangeContainer {
                        data: DataType::Basic(BasicDataType::Int),
                        position: PositionRange {
                            line: 1,
                            column: 42..=44,
                        },
                    },
                },
            ],
        }),
    ))];
    assert!(parser.eq(expected));
}

#[test]
fn parse_binary_operation() {
    let parser = parser::sourcecode_to_parser("1 + 2 * 3".chars());
    let expected: [Result<AstNode, FTLError>; 1] =
        [Ok(AstNode::Statement(Statement::Function(Function {
            prototype: FunctionPrototype {
                name: PositionRangeContainer {
                    data: String::from("__main_line_1"),
                    position: PositionRange {
                        line: 1,
                        column: 1..=1,
                    },
                },
                args: vec![],
            },
            body: Expression::BinaryExpression(BinaryExpression {
                lhs: Box::new(Expression::Number(PositionRangeContainer {
                    data: 1.0,
                    position: PositionRange {
                        line: 1,
                        column: 1..=1,
                    },
                })),
                operator: PositionRangeContainer {
                    data: BinaryOperator::Add,
                    position: PositionRange {
                        line: 1,
                        column: 3..=3,
                    },
                },
                rhs: Box::new(Expression::BinaryExpression(BinaryExpression {
                    lhs: Box::new(Expression::Number(PositionRangeContainer {
                        data: 2.0,
                        position: PositionRange {
                            line: 1,
                            column: 5..=5,
                        },
                    })),
                    operator: PositionRangeContainer {
                        data: BinaryOperator::Multiply,
                        position: PositionRange {
                            line: 1,
                            column: 7..=7,
                        },
                    },
                    rhs: Box::new(Expression::Number(PositionRangeContainer {
                        data: 3.0,
                        position: PositionRange {
                            line: 1,
                            column: 9..=9,
                        },
                    })),
                })),
            }),
        })))];
    assert!(parser.eq(expected));
}
