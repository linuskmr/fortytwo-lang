use crate::token::{Token, TokenKind};
use core::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum Error {
	ExpectedToken {
		expected: TokenKind,
		found: Option<Token>,
	},

	IllegalToken {
		token: Option<Token>,
		context: &'static str,
	},
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::ExpectedToken { expected, found } => match found {
				Some(token) => write!(
					f,
					"{} Expected token {:?}, found {:?}",
					token.position, expected, token.inner
				),
				None => write!(f, "Expected token {:?}, found nothing", expected),
			},
			Error::IllegalToken { token, context } => match token {
				Some(token) => write!(
					f,
					"{} Illegal token '{:?}' in {}",
					token.position, token.inner, context
				),
				None => write!(f, "Illegal token in {}", context),
			},
		}
	}
}
