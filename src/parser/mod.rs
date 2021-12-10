//! The parser parses the tokens created by the lexer and and builds an abstract syntax tree
//! from them.

mod error;

use miette::{NamedSource, SourceSpan};
use std::convert::{TryFrom, TryInto};
use std::iter::Peekable;
use std::sync::Arc;

use crate::ast::*;

use crate::position_container::PositionContainer;

use crate::token::{Token, TokenKind};
use crate::{ast, iter_advance_while};

/// A parser of tokens generated by its [Lexer].
pub struct Parser<TokenIter: Iterator<Item = Token>> {
    /// The source to read the [Token]s from.
    tokens: Peekable<TokenIter>,
    named_source: Arc<NamedSource>,
}

impl<TokenIter: Iterator<Item = Token>> Parser<TokenIter> {
    /// Creates a new Parser from the given token iterator.
    pub fn new(tokens: TokenIter, named_source: Arc<NamedSource>) -> Self {
        Self {
            tokens: tokens.peekable(),
            named_source,
        }
    }

    /// Returns the position of the current token or (0, 0) if self.tokens.peek() returns [None].
    fn current_position(&mut self) -> SourceSpan {
        self.tokens
            .peek()
            .map(|token| token.position.clone())
            .unwrap_or(SourceSpan::new(0.into(), 0.into()))
    }

    /// Parses a binary expression, potentially followed by a sequence of (binary operator, primary expression).
    ///
    /// Note: Parentheses are a primary expression, so we don't have to worry about them here.
    fn parse_binary_expression(&mut self) -> miette::Result<Expression> {
        let lhs: Expression = self.parse_primary_expression()?;
        self.parse_binary_operation_rhs(None, lhs)
    }

    /// Parses a sequence of `(binary operator, primary expression)`. If this sequence is empty, it returns `lhs`. If
    /// the binary operator has less precedence than `min_operator`.
    ///
    /// # Examples
    ///
    /// Think of the following expression: `a + b * c`. Then `lhs` contains `a`. This function reads the
    /// operator `+` and parses the following expression as `rhs`, so `b` here. Than `next_operator` is read and
    /// contains `*`. Because [BinaryOperator::Multiplication] (`*`) has a higher precedence than
    /// [BinaryOperator::Addition] (`+`). This causes this function recursively
    /// calls itself and parses everything on the right side until an operator is found, which precedence is not
    /// higher than `+`.
    fn parse_binary_operation_rhs(
        &mut self,
        min_operator: Option<&BinaryOperator>,
        lhs: Expression,
    ) -> miette::Result<Expression> {
        // Make lhs mutable without enforcing the function caller that its lhs must be mutable
        let mut lhs = lhs;
        loop {
            // Read the operator after lhs and before rhs. On Err(...), return the error
            let operator = match self.parse_operator(min_operator, true) {
                // Found an operator
                Some(operator) => operator,
                // Expression ended here or the next operator does not bind strong enough with lhs
                None => return Ok(lhs),
            };
            // Parse the primary expression after operator as rhs
            let mut rhs: Expression = self.parse_primary_expression()?;
            // Inspect next operator. If `next_operator` binds stronger with `rhs` than the current `operator`,
            // let `rhs` go with `next_operator`
            if let Some(_) = self.parse_operator(Some(&operator.data), false) {
                rhs = self.parse_binary_operation_rhs(Some(&operator.data), rhs)?;
            }
            // Merge lhs and rhs and continue parsing
            lhs = Expression::BinaryExpression(BinaryExpression {
                lhs: Box::new(lhs),
                operator,
                rhs: Box::new(rhs),
            });
        }
    }

    /// Parses the next [BinaryOperator] from [Lexer::tokens]. Returns the [BinaryOperator] if it has more precedence than
    /// `min_operator`, otherwise [None].
    ///
    /// # Arguments
    ///
    /// * `min_operator` - The parsed operator has to be greater than this minimum threshold. If [None], accept all
    /// operators.
    /// * `consume` - True if you want that the operator gets consumed, i.e. [Lexer::tokens.next()] will not yield the
    /// operator, but the token after the operator. False if you want that the operator don't gets consumed, i.e.
    /// [Lexer::tokens.next()] will yield the operator.
    fn parse_operator(&mut self, min_operator: Option<&BinaryOperator>, consume: bool) -> Option<PositionContainer<BinaryOperator>> {
        // Read the operator
        let operator = match self.tokens.peek() {
            // No operator
            Some(Token { data: TokenKind::EndOfLine, .. }) | None => return None,
            Some(token) => PositionContainer {
                data: token.data.clone().try_into().ok()?,
                position: token.position.clone(),
            },
        };
        // Consume operator
        if consume {
            self.tokens.next();
        }
        match min_operator {
            // min_operator not set. Accept every operator
            None => Some(operator),
            // Do not take operator with less or equal precedence compared to min_operator
            Some(min_op) => {
                if &operator.data > min_op {
                    Some(operator)
                } else {
                    None
                }
            }
        }
    }

    /// Parses a [FunctionPrototype], i.e. a function name followed by opening parentheses, a list of arguments and
    /// closing parentheses.
    ///
    /// # Examples
    ///
    /// A valid function prototype is:
    /// ```text
    /// foo(x: int, y: float)
    /// ```
    fn parse_function_prototype(&mut self) -> miette::Result<FunctionPrototype> {
        // Get and consume function name
        let name = match self.tokens.next() {
            Some(Token {
                data: TokenKind::Identifier(identifier),
                position,
            }) => PositionContainer {
                data: identifier,
                position,
            },
            other => {
                return Err(error::ExpectedIdentifier {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        };
        // Check opening parentheses
        match self.tokens.peek() {
            Some(Token {
                data: TokenKind::OpeningParentheses,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedOpeningRoundParentheses {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position.clone())
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        }
        // Read list of arguments
        let args = self.parse_function_argument_type_list()?;
        // TODO: Add parsing for the return value
        Ok(FunctionPrototype { name, args })
    }

    /// Parses a comma seperated list of arguments with their type. This function is used when parsing the arguments
    /// of a [FunctionPrototype], *not* when parsing a [FunctionCall].
    ///
    /// This function consumes a [TokenKind::OpeningParentheses] (and panics if this is not the case) and then reads
    /// the arguments with their types. When no [TokenKind::Comma] follows, the argument list ends and an
    /// [TokenKind::ClosingParentheses] is expected.
    ///
    /// # Examples
    ///
    /// A valid argument list is:
    /// ```text
    /// (x: int, y: float)
    /// ```
    fn parse_function_argument_type_list(&mut self) -> miette::Result<Vec<FunctionArgument>> {
        // Check and consume opening parentheses
        assert_eq!(
            self.tokens.next().map(|token| token.data),
            Some(TokenKind::OpeningParentheses)
        );
        let mut arguments = Vec::new();
        // Check if argument list starts with a closing parentheses `)`. If yes, the argument list is finished
        if let Some(Token {
            data: TokenKind::ClosingParentheses,
            ..
        }) = self.tokens.peek()
        {
            self.consume_closing_parentheses()?;
            return Ok(arguments);
        }
        // Collect all arguments
        loop {
            // Get and consume argument name
            let name = match self.tokens.next() {
                Some(Token {
                    data: TokenKind::Identifier(data),
                    position,
                }) => PositionContainer { data, position },
                other => {
                    return Err(error::ExpectedArgumentName {
                        src: self.named_source.clone(),
                        err_span: other
                            .map(|token| token.position)
                            .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                    }
                    .into());
                }
            };
            // Check and consume colon
            match self.tokens.next() {
                Some(Token {
                    data: TokenKind::Colon,
                    ..
                }) => (),
                other => {
                    return Err(error::ExpectedColonBetweenNameAndType {
                        src: self.named_source.clone(),
                        err_span: other
                            .map(|token| token.position)
                            .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                    }
                    .into());
                }
            };
            // Get and consume argument type
            let data_type = self.parse_type()?;
            arguments.push(FunctionArgument { name, data_type });
            // Check and consume comma
            match self.tokens.peek() {
                Some(Token {
                    data: TokenKind::Comma,
                    ..
                }) => self.tokens.next(),
                _ => break, // No comma after this argument means this is the last argument
            };
        }
        self.consume_closing_parentheses()?;
        Ok(arguments)
    }

    /// Parses a [DataType]. A [DataType] is either a
    /// * basic data type (like `int` or `float`),
    /// * pointer to a data type (like `ptr int`),
    /// * user defined data type / struct (like `Person`).
    fn parse_type(&mut self) -> miette::Result<PositionContainer<DataType>> {
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::Identifier(type_str),
                position,
            }) if type_str == "ptr" => {
                // Pointer
                // Recursively call parse_type() to parse the type the pointer points to. This recursive calling
                // enables types like `ptr ptr int`.
                let type_to_point_to = self.parse_type()?;
                Ok(PositionContainer {
                    data: DataType::Pointer(Box::new(type_to_point_to.clone())),
                    position: SourceSpan::new(
                        position.offset().into(),
                        type_to_point_to.position.len().into(),
                    ),
                })
            }
            Some(Token {
                data: TokenKind::Identifier(type_str),
                position,
            }) => {
                if let Ok(basic_data_type) = BasicDataType::try_from(type_str.as_str()) {
                    // Basic data type
                    Ok(PositionContainer {
                        data: ast::DataType::Basic(basic_data_type),
                        position,
                    })
                } else {
                    // User defined data type / struct
                    Ok(PositionContainer {
                        data: DataType::Struct(type_str),
                        position,
                    })
                }
            }
            other => {
                return Err(error::ExpectedArgumentType {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        }
    }

    /// Parses a [Function] definition, i.e. a [FunctionPrototype] followed by the function body (an [Expression]).
    fn parse_function_definition(&mut self) -> miette::Result<Function> {
        // Check and consume function definition
        assert_eq!(
            self.tokens.next().map(|token| token.data),
            Some(TokenKind::FunctionDefinition)
        );
        let prototype = self.parse_function_prototype()?;
        let body = self.parse_binary_expression()?;
        return Ok(Function { prototype, body });
    }

    /// Parses a [Number], i.e. simply converts a [TokenKind::Number] from [Lexer::tokens.next()] to an [Number].
    ///
    /// # Panics
    ///
    /// Panics if [Lexer::tokens.next()] yields a [Token] which has not [TokenKind::Number], so test this before
    /// calling this function with [Lexer::tokens.peek()]
    fn parse_number(&mut self) -> miette::Result<PositionContainer<f64>> {
        Ok(match self.tokens.next() {
            Some(Token {
                data: TokenKind::Number(number),
                position,
            }) => PositionContainer {
                data: number,
                position,
            },
            _ => panic!("parse_number(): Expected number"),
        })
    }

    /// Parses a parentheses expression, i.e. a [TokenKind::OpeningParentheses] followed by an inner [Expression] and
    /// a final [TokenKind::ClosingParentheses]. The parentheses are not present in the AST, but are expressed by the
    /// AST structure.
    ///
    /// # Examples
    ///
    /// Valid parentheses expression are:
    /// ```text
    /// (40 + 2)
    /// (42 - answer_to_everything + 42)
    /// ```
    ///
    /// Not valid parentheses expression are:
    /// ```text
    /// (40 +2
    /// 40 + 2)
    /// ```
    fn parse_parentheses(&mut self) -> miette::Result<Expression> {
        assert_eq!(
            self.tokens.next().map(|token| token.data),
            Some(TokenKind::OpeningParentheses)
        );
        let inner_expression = self.parse_binary_expression()?;
        self.consume_closing_parentheses()?;
        return Ok(inner_expression);
    }

    /// Parses a variable, i.e. does checks on the provided `identifier` and if they were successful, returns it.
    fn parse_variable(
        &mut self,
        identifier: PositionContainer<String>,
    ) -> miette::Result<PositionContainer<String>> {
        assert!(!identifier.data.is_empty()); // identifier can't be empty, because who should produce an empty token?
        Ok(identifier)
    }

    /// Parses an extern function, i.e. an [TokenKind::Extern] followed by a [FunctionPrototype] without a body.
    ///
    /// # Examples
    ///
    /// A valid declaration of an extern function for the
    /// [write syscall in libc](https://man7.org/linux/man-pages/man2/write.2.html) is:
    /// ```text
    /// extern write(fd: int, buf: ptr char, count: uint64)
    /// ```
    fn parse_extern_function(&mut self) -> miette::Result<ast::FunctionPrototype> {
        assert_eq!(
            self.tokens.next().map(|token| token.data),
            Some(TokenKind::Extern)
        );
        self.parse_function_prototype()
    }

    /// Parses a top level expression, so this is the entry point of an ftl program. In the moment an ftl program is
    /// only one binary expression, which gets wrapped in a main function. This will change in the future.
    fn parse_top_level_expression(&mut self) -> miette::Result<Function> {
        let body = self.parse_binary_expression()?;
        let prototype = FunctionPrototype {
            name: PositionContainer {
                data: format!("__anonymous_offset{}", self.current_position().offset()),
                position: self.current_position(),
            },
            args: Vec::new(),
        };
        Ok(Function { prototype, body })
    }

    /// Parses a comma seperated list of expressions ended with a [TokenKind::ClosingParentheses]. This function is
    /// used when parsing a [FunctionCall], *not* when parsing a [FunctionPrototype].
    ///
    /// This function consumes a [TokenKind::OpeningParentheses] (and panics if this is not the case) and then reads
    /// the parameters as expressions. When no [TokenKind::Comma] follows, the argument list ends and an
    /// [TokenKind::ClosingParentheses] is expected.
    fn parse_function_parameters(&mut self) -> miette::Result<Vec<Expression>> {
        // Check and consume opening parentheses
        assert_eq!(
            self.tokens.next().map(|token| token.data),
            Some(TokenKind::OpeningParentheses)
        );
        let mut parameters = Vec::new();
        // Check if argument list starts with an closing parentheses `)`. If yes, the argument list is finished.
        if let Some(Token {
            data: TokenKind::ClosingParentheses,
            ..
        }) = self.tokens.peek()
        {
            self.consume_closing_parentheses()?;
            return Ok(parameters);
        };
        // Collect all parameters
        loop {
            let argument: Expression = self.parse_primary_expression()?;
            parameters.push(argument);
            // Check and consume comma
            match self.tokens.peek() {
                Some(Token {
                    data: TokenKind::Comma,
                    ..
                }) => self.tokens.next(),
                _ => break, // No comma after this argument means this is the last argument
            };
        }
        self.consume_closing_parentheses()?;
        Ok(parameters)
    }

    /// Checks and consumes a [TokenKind::ClosingParentheses] after a argument/parameter list.
    fn consume_closing_parentheses(&mut self) -> miette::Result<()> {
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::ClosingParentheses,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedClosingRoundParentheses {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        }
        Ok(())
    }

    /// Parses a function call expression, like `add(2, 3)`.
    fn parse_function_call(
        &mut self,
        name: PositionContainer<String>,
    ) -> miette::Result<FunctionCall> {
        let params = self.parse_function_parameters()?;
        Ok(FunctionCall { name, params })
    }

    /// Parses an identifier. The output is either a [ast::Expression::FunctionCall] or an [ast::Expression::Variable].
    fn parse_identifier_expression(&mut self) -> miette::Result<ast::Expression> {
        let identifier = match self.tokens.next() {
            Some(Token {
                data: TokenKind::Identifier(identifier),
                position,
            }) => PositionContainer {
                data: identifier,
                position: position.into(),
            },
            _ => panic!("parse_identifier_expression() called on non-identifier"),
        };
        match self.tokens.peek() {
            Some(Token {
                data: TokenKind::OpeningParentheses,
                ..
            }) => {
                // Identifier is followed by an opening parentheses, so it must be a function call
                let function_call = self.parse_function_call(identifier)?;
                Ok(Expression::FunctionCall(function_call))
            }
            _ => {
                // Identifier is followed by something else, so it is a variable
                let variable = self.parse_variable(identifier)?;
                Ok(Expression::Variable(variable))
            }
        }
    }

    /// Parses a for expression. See [ForExpression] for details.
    fn parse_for_expression(&mut self) -> miette::Result<ForLoop> {
        // Consume for
        assert_eq!(
            self.tokens.next().map(|token| token.data),
            Some(TokenKind::For)
        );
        // Read setup
        let setup = self.parse_binary_expression()?;
        // Check and consume semicolon `;`
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::Semicolon,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedSemicolon {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        };
        // Read condition
        let condition = self.parse_binary_expression()?;
        // Check and consume semicolon `;`
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::Semicolon,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedSemicolon {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        };
        // Read advancement
        let advancement = self.parse_binary_expression()?;
        // Check and consume opening curly braces `{`
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::OpeningCurlyBraces,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedOpeningCurlyParentheses {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                    after: "for loop header".to_string(),
                }
                .into());
            }
        };
        self.skip_newlines();
        // Read body
        let body = self.parse_binary_expression()?;
        // Check and consume closing curly braces `}`
        self.skip_newlines();
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::ClosingCurlyBraces,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedClosingCurlyParentheses {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        };
        Ok(ForLoop {
            setup,
            condition,
            advancement,
            body,
        })
    }

    /// Parses an if expression. See [IfExpression] for details.
    fn parse_if_expression(&mut self) -> miette::Result<IfElseExpression> {
        // Consume if
        assert_eq!(
            self.tokens.next().map(|token| token.data),
            Some(TokenKind::If)
        );
        // Read condition
        let condition = self.parse_binary_expression()?;
        // Check and consume opening curly braces `{`
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::OpeningCurlyBraces,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedOpeningCurlyParentheses {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                    after: "if condition".to_string(),
                }
                .into());
            }
        };
        self.skip_newlines();
        // Parse expression to execute if condition is true
        let if_true = self.parse_binary_expression()?;
        self.skip_newlines();
        // Check and consume closing curly braces `}`
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::ClosingCurlyBraces,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedClosingCurlyParentheses {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        };
        // Check and consume else
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::Else,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedElse {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        };
        // Check and consume opening curly braces `{`
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::OpeningCurlyBraces,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedOpeningCurlyParentheses {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                    after: "else".to_string(),
                }
                .into());
            }
        };
        self.skip_newlines();
        // Parse expression to execute if condition is false
        let if_false = self.parse_binary_expression()?;
        self.skip_newlines();
        // Check and consume closing curly braces `}`
        match self.tokens.next() {
            Some(Token {
                data: TokenKind::ClosingCurlyBraces,
                ..
            }) => (),
            other => {
                return Err(error::ExpectedClosingCurlyParentheses {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position)
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                }
                .into());
            }
        };
        Ok(IfElseExpression {
            condition,
            if_true,
            if_false,
        })
    }

    /// Skips all newlines in [Parser::tokens].
    fn skip_newlines(&mut self) {
        iter_advance_while(&mut self.tokens, |token| match token {
            Token {
                data: TokenKind::EndOfLine,
                ..
            } => true,
            _ => false,
        });
    }

    /// Parses the most basic type of an expression, i.e. looks at whether an identifier, number or parentheses is
    /// yielded by [Lexer::tokens] and calls the appropriate parsing function.
    ///
    /// # Examples
    ///
    /// Identifier and function calls (calls [Parser::parse_identifier_expression()]):
    /// ```text
    /// foo
    /// foo()
    /// ```
    ///
    /// Number (calls [Parser::parse_number()]):
    /// ```text
    /// 42
    /// ```
    ///
    /// Parentheses (calls [Parser::parse_parentheses()]):
    /// ```text
    /// (3 + 7)
    /// ```
    fn parse_primary_expression(&mut self) -> miette::Result<Expression> {
        match self.tokens.peek() {
            Some(Token {
                data: TokenKind::Identifier(_),
                ..
            }) => self.parse_identifier_expression(),
            Some(Token {
                data: TokenKind::Number(_),
                ..
            }) => Ok(Expression::Number(self.parse_number()?)),
            Some(Token {
                data: TokenKind::OpeningParentheses,
                ..
            }) => self.parse_parentheses(),
            Some(Token {
                data: TokenKind::If,
                ..
            }) => Ok(Expression::IfElse(Box::new(self.parse_if_expression()?))),
            Some(Token {
                data: TokenKind::For,
                ..
            }) => Ok(Expression::ForLoop(Box::new(self.parse_for_expression()?))),
            other => {
                return Err(error::ExpectedExpression {
                    src: self.named_source.clone(),
                    err_span: other
                        .map(|token| token.position.clone())
                        .unwrap_or(SourceSpan::new(0.into(), 0.into())), // TODO: Better position
                    help_msg: format!("Extern functions don't have a body"),
                }
                .into());
            }
        }
    }
}

impl<L: Iterator<Item = Token>> Iterator for Parser<L> {
    type Item = miette::Result<AstNode>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokens.peek()? {
            Token {
                data: TokenKind::FunctionDefinition,
                ..
            } => Some(match self.parse_function_definition() {
                Ok(def) => Ok(AstNode::Statement(Statement::Function(def))),
                Err(err) => Err(err),
            }),
            Token {
                data: TokenKind::Extern,
                ..
            } => Some(match self.parse_extern_function() {
                Ok(extern_function) => Ok(AstNode::Statement(Statement::FunctionPrototype(
                    extern_function,
                ))),
                Err(err) => Err(err),
            }),
            Token {
                data: TokenKind::EndOfLine,
                ..
            } => {
                // No_op (No operation)
                self.tokens.next();
                // TODO: This accumulates a large stack during parsing. Try to do this with a loop.
                self.next()
            }
            _ => Some(match self.parse_top_level_expression() {
                Ok(expression) => Ok(AstNode::Statement(Statement::Function(expression))),
                Err(err) => Err(err),
            }),
        }
    }
}
