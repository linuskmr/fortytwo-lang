//! Common helper functions for tests.
//! For more information see https://doc.rust-lang.org/book/ch11-03-test-organization.html#submodules-in-integration-tests

use crate::error::ParseResult;
use crate::lexer::Lexer;
use crate::parser::Parser;

use crate::position_reader::PositionReader;
use crate::token::Token;
use std::iter::Map;
use std::str::Chars;


