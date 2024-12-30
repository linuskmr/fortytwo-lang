use std::sync::Arc;
use crate::lexer::*;
use crate::source::Source;
use crate::token::Token;


/// Tests that the lexer can read a string literal containing escaped quotes.
#[test]
fn test_read_string_literal() {
    let tokens = lexer(r#""hello \"name\"!""#);
    assert_eq!(tokens[0].value, TokenKind::StringLiteral(r#"hello "name"!"#.to_owned()));
}

/// Tests that the lexer can read an identifier.
#[test]
fn test_read_identifier() {
    let tokens = lexer("hello");
    assert_eq!(tokens[0].value, TokenKind::Identifier("hello".to_owned()));
}

/// Tests that the lexer can read a float.
#[test]
fn test_read_int() {
    let tokens = lexer("42");
    assert_eq!(tokens[0].value, TokenKind::Int(42));
}

/// Tests that the lexer can read a float.
#[test]
fn test_read_float() {
    let tokens = lexer("4.2");
    assert_eq!(tokens[0].value, TokenKind::Float(4.2));
}


/// Boilerplate code for converting source code into tokens using a lexer.
fn lexer(source_code: &str) -> Vec<Token> {
    let source = Arc::new(Source::new("testfile".to_owned(), source_code.to_owned()));
    let lexer = Lexer::new(source.iter());
    lexer.collect::<Result<Vec<Token>, Error>>().unwrap()
}