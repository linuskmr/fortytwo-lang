use super::{Error, Result};
use crate::ast::expression::BinaryOperator;
use crate::source::PositionContainer;
use crate::token::{Token, TokenKind};

pub(crate) fn parse_identifier(token: Option<Token>) -> Result<PositionContainer<String>> {
    match token {
        Some(Token {
            position,
            inner: TokenKind::Identifier(ident),
        }) => Ok(PositionContainer::new(ident, position)),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::Identifier(String::new()),
            found: token,
        }),
    }
}

pub(crate) fn parse_opening_parenthesis(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::OpeningParentheses) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::OpeningParentheses,
            found: token,
        }),
    }
}

pub(crate) fn parse_closing_parenthesis(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::ClosingParentheses) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::ClosingParentheses,
            found: token,
        }),
    }
}

pub(crate) fn parse_colon(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::Colon) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::Colon,
            found: token,
        }),
    }
}

pub(crate) fn parse_comma(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::Comma) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::Comma,
            found: token,
        }),
    }
}

pub(crate) fn parse_semicolon(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::Semicolon) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::Semicolon,
            found: token,
        }),
    }
}

pub(crate) fn parse_opening_curly_parenthesis(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::OpeningCurlyBraces) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::OpeningCurlyBraces,
            found: token,
        }),
    }
}

pub(crate) fn parse_closing_curly_parenthesis(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::ClosingCurlyBraces) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::ClosingCurlyBraces,
            found: token,
        }),
    }
}

pub(crate) fn parse_variable_declaration(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::Var) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::Var,
            found: token,
        }),
    }
}

pub(crate) fn parse_equal(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::Equal) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::Equal,
            found: token,
        }),
    }
}

pub(crate) fn parse_if(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::If) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::If,
            found: token,
        }),
    }
}

pub(crate) fn parse_struct(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::Struct) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::Struct,
            found: token,
        }),
    }
}

pub(crate) fn parse_while(token: Option<Token>) -> Result<()> {
    match token.as_deref() {
        Some(TokenKind::While) => Ok(()),
        _ => Err(Error::ExpectedToken {
            expected: TokenKind::While,
            found: token,
        }),
    }
}

pub(crate) fn parse_operator(token: Option<Token>) -> Result<PositionContainer<BinaryOperator>> {
    match token {
        Some(token) => Ok(PositionContainer {
            position: token.position.clone(),
            inner: match token.inner {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Subtract,
                TokenKind::Star => BinaryOperator::Multiply,
                TokenKind::Slash => BinaryOperator::Divide,
                TokenKind::Equal => BinaryOperator::Equal,
                TokenKind::NotEqual => BinaryOperator::NotEqual,
                TokenKind::Less => BinaryOperator::Less,
                // TokenKind::LessEqual => BinaryOperator::LessEqual,
                TokenKind::Greater => BinaryOperator::Greater,
                // TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
                _ => {
                    return Err(Error::ExpectedToken {
                        expected: TokenKind::Plus,
                        found: Some(token),
                    })
                }
            },
        }),
        None => Err(Error::IllegalToken {
            token,
            context: "operator",
        }),
    }
}
