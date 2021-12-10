/*use crate::ast::DataType::Basic;
use crate::ast::{
    AstNode, BasicDataType, BinaryExpression, BinaryOperator, DataType, Expression, Function,
    FunctionArgument, FunctionCall, FunctionPrototype, Statement,
};

use crate::parser;
use crate::position_container::{PositionContainer};

#[test]
fn parse_extern() {
    let parser =
        parser::sourcecode_to_parser("extern write(fd: int, buf: ptr int, len: int)".chars());
    let expected: [Result<AstNode, FTLError>; 1] = [Ok(AstNode::Statement(
        Statement::FunctionPrototype(FunctionPrototype {
            name: PositionContainer {
                data: String::from("write"),
                position: PositionRange {
                    line: 1,
                    column: 8..=12,
                },
            },
            args: vec![
                FunctionArgument {
                    name: PositionContainer {
                        data: String::from("fd"),
                        position: PositionRange {
                            line: 1,
                            column: 14..=15,
                        },
                    },
                    data_type: PositionContainer {
                        data: DataType::Basic(BasicDataType::Int),
                        position: PositionRange {
                            line: 1,
                            column: 18..=20,
                        },
                    },
                },
                FunctionArgument {
                    name: PositionContainer {
                        data: String::from("buf"),
                        position: PositionRange {
                            line: 1,
                            column: 23..=25,
                        },
                    },
                    data_type: PositionContainer {
                        data: DataType::Pointer(Box::new(PositionContainer {
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
                    name: PositionContainer {
                        data: String::from("len"),
                        position: PositionRange {
                            line: 1,
                            column: 37..=39,
                        },
                    },
                    data_type: PositionContainer {
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
    let sourcecode = "1 + 2 * (1 - 4)";
    let parser = parser::sourcecode_to_parser(sourcecode.chars());
    let expected: [Result<AstNode, FTLError>; 1] =
        [Ok(AstNode::Statement(Statement::Function(Function {
            prototype: FunctionPrototype {
                name: PositionContainer {
                    data: String::from("__main_line_1"),
                    position: PositionRange {
                        line: 1,
                        column: 1..=1,
                    },
                },
                args: vec![],
            },
            body: Expression::BinaryExpression(BinaryExpression {
                lhs: Box::new(Expression::Number(PositionContainer {
                    data: 1.0,
                    position: PositionRange {
                        line: 1,
                        column: 1..=1,
                    },
                })),
                operator: PositionContainer {
                    data: BinaryOperator::Add,
                    position: PositionRange {
                        line: 1,
                        column: 3..=3,
                    },
                },
                rhs: Box::new(Expression::BinaryExpression(BinaryExpression {
                    lhs: Box::new(Expression::Number(PositionContainer {
                        data: 2.0,
                        position: PositionRange {
                            line: 1,
                            column: 5..=5,
                        },
                    })),
                    operator: PositionContainer {
                        data: BinaryOperator::Multiply,
                        position: PositionRange {
                            line: 1,
                            column: 7..=7,
                        },
                    },
                    rhs: Box::new(Expression::BinaryExpression(BinaryExpression {
                        lhs: Box::new(Expression::Number(PositionContainer {
                            data: 1.0,
                            position: PositionRange {
                                line: 1,
                                column: 10..=10,
                            },
                        })),
                        operator: PositionContainer {
                            data: BinaryOperator::Subtract,
                            position: PositionRange {
                                line: 1,
                                column: 12..=12,
                            },
                        },
                        rhs: Box::new(Expression::Number(PositionContainer {
                            data: 4.0,
                            position: PositionRange {
                                line: 1,
                                column: 14..=14,
                            },
                        })),
                    })),
                })),
            }),
        })))];
    assert!(parser.eq(expected));
}

#[test]
fn parse_function_call() {
    let sourcecode = "add(42, random())";
    let parser = parser::sourcecode_to_parser(sourcecode.chars());
    let expected: [Result<AstNode, FTLError>; 1] =
        [Ok(AstNode::Statement(Statement::Function(Function {
            prototype: FunctionPrototype {
                name: PositionContainer {
                    data: String::from("__main_line_1"),
                    position: PositionRange {
                        line: 1,
                        column: 1..=1,
                    },
                },
                args: vec![],
            },
            body: Expression::FunctionCall(FunctionCall {
                name: PositionContainer {
                    data: String::from("add"),
                    position: PositionRange {
                        line: 1,
                        column: 1..=3,
                    },
                },
                params: vec![
                    Expression::Number(PositionContainer {
                        data: 42.0,
                        position: PositionRange {
                            line: 1,
                            column: 5..=6,
                        },
                    }),
                    Expression::FunctionCall(FunctionCall {
                        name: PositionContainer {
                            data: String::from("random"),
                            position: PositionRange {
                                line: 1,
                                column: 9..=14,
                            },
                        },
                        params: vec![],
                    }),
                ],
            }),
        })))];
    assert!(parser.eq(expected));
}
*/