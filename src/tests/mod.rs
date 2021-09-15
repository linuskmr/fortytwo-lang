//! Integration tests for fortytwo-lang.

mod common;
mod test_parser;




use crate::lexer::Lexer;


use crate::position_container::{PositionRange, PositionRangeContainer};
use crate::position_reader::PositionReader;
use crate::token::TokenKind::*;

#[test]
fn lexer() {
    let sourcecode = "1 + 2\n\ndef abc(x, y)";
    let position_reader = PositionReader::new(sourcecode.chars());
    let lexer = Lexer::new(position_reader);
    let expected = [
        Ok(PositionRangeContainer {
            data: Number(1.0),
            position: PositionRange {
                line: 1,
                column: 1..=1,
            },
        }),
        Ok(PositionRangeContainer {
            data: Plus,
            position: PositionRange {
                line: 1,
                column: 3..=3,
            },
        }),
        Ok(PositionRangeContainer {
            data: Number(2.0),
            position: PositionRange {
                line: 1,
                column: 5..=5,
            },
        }),
        Ok(PositionRangeContainer {
            data: EndOfLine,
            position: PositionRange {
                line: 1,
                column: 6..=6,
            },
        }),
        Ok(PositionRangeContainer {
            data: EndOfLine,
            position: PositionRange {
                line: 2,
                column: 1..=1,
            },
        }),
        Ok(PositionRangeContainer {
            data: FunctionDefinition,
            position: PositionRange {
                line: 3,
                column: 1..=3,
            },
        }),
        Ok(PositionRangeContainer {
            data: Identifier(String::from("abc")),
            position: PositionRange {
                line: 3,
                column: 5..=7,
            },
        }),
        Ok(PositionRangeContainer {
            data: OpeningParentheses,
            position: PositionRange {
                line: 3,
                column: 8..=8,
            },
        }),
        Ok(PositionRangeContainer {
            data: Identifier(String::from("x")),
            position: PositionRange {
                line: 3,
                column: 9..=9,
            },
        }),
        Ok(PositionRangeContainer {
            data: Comma,
            position: PositionRange {
                line: 3,
                column: 10..=10,
            },
        }),
        Ok(PositionRangeContainer {
            data: Identifier(String::from("y")),
            position: PositionRange {
                line: 3,
                column: 12..=12,
            },
        }),
        Ok(PositionRangeContainer {
            data: ClosingParentheses,
            position: PositionRange {
                line: 3,
                column: 13..=13,
            },
        }),
    ];
    assert!(lexer.eq(expected));
}
