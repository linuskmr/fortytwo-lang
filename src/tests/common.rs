//! Common helper functions for tests.
//! For more information see https://doc.rust-lang.org/book/ch11-03-test-organization.html#submodules-in-integration-tests

use crate::position_reader::PositionReader;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::position_container::PositionRangeContainer;
use std::str::Chars;
use crate::token::Token;
use std::iter::Map;
use crate::error::{FTLError, ParseResult};

/// Converts the `sourcecode` to a parser. Don't care about the weird return type. It's simply a parser.
pub(crate) fn sourcecode_to_parser(
    sourcecode: &str
) -> Parser<Map<Lexer<PositionReader<Chars<'_>>>, fn(ParseResult<Token>) -> Token>> {
    let position_reader = PositionReader::new(sourcecode.chars());
    let lexer = Lexer::new(position_reader);
    // Result::unwrap as fn(ParseResult<Token>) -> Token: Convert fn item to fn pointer.
    // See https://users.rust-lang.org/t/puzzling-expected-fn-pointer-found-fn-item/46423/4
    let token_iter = lexer.map(Result::unwrap as fn(ParseResult<Token>) -> Token);
    Parser::new(token_iter)
}