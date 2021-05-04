use std::io;
use std::path::Iter;
use std::process::id;

use ast::{AST, Variable};
use lexer::{Lexer, Token, TokenType};

struct Parser<R: io::Read> {
    lexer: Lexer<R>,
    current_token: Option<Token>,
}

type ParseResult = Result<Box<dyn ast::AST>, String>;

impl<R: io::Read> Parser<R> {
    /// Creates a new Parser with the lexer.
    fn new(lexer: Lexer<R>) -> Self {
        Self {
            lexer,
            current_token: None,
        }
    }

    /// Reads the next token from self.lexer, writes it into self.current_token and returns it.
    fn get_next_token(&mut self) -> &Option<Token> {
        self.current_token = self.lexer.next();
        &self.current_token
    }

    fn parse_expression(&mut self) -> ParseResult {
        Ok(Box::new(ast::Number(42.0)))
    }

    /// Converts a number to a ast::Number.
    fn parse_number(&mut self, number: f64) -> ast::Number {
        ast::Number(number)
    }

    fn parse_parentheses(&mut self) -> ParseResult {
        self.get_next_token(); // Eat (
        let inner_expression = self.parse_expression()?;
        if self.current_token != ')' {
            return Err(format!("Expected `)`"));
        }
        self.get_next_token(); // Eat )
        return Ok(inner_expression);
    }

    fn parse_variable(&mut self, variable_name: String) -> ParseResult {
        Ok(Box::new(ast::Variable(identifier)))
    }

    fn collect_function_call_arguments(&mut self) -> Result<Vec<Box<dyn ast::AST>>, String> {
        if self.current_token == ')' {
            // No arguments were passed
            return Ok(Vec::new());
        }
        let mut args = Vec::new();
        loop {
            args.push(self.parse_expression()?);
            match self.current_token {
                Token { token_type: TokenType::Other(')'), .. } => {
                    // End of argument list
                    break;
                }
                Token { token_type: TokenType::Other(','), .. } => {
                    // Ok, the argument list keeps going
                    ()
                }
                _ => {
                    // Illegal token
                    return Err(format!("Expected `)` or `,` in argument list"));
                }
            };
        }
        Ok(args)
    }

    fn parse_function_call(&mut self, function_name: String) -> ParseResult {
        self.get_next_token(); // Eat (
        let mut args = self.collect_function_call_arguments()?;
        self.get_next_token(); // Eat )
        Ok(Box::new(ast::FunctionCall { function_name, args }))
    }

    fn parse_identifier(&mut self, identifier: String) -> ParseResult {
        match self.get_next_token().unwrap() { // TODO: Error handling
            Token { token_type: TokenType::Other('('), .. } => {
                self.parse_function_call(identifier)
            }
            _ => self.parse_variable(identifier)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let code = "abc +12.3 -4 def";
        let code = code.as_bytes();
        let l = Lexer::new(code);
    }
}