use std::sync::Arc;
use miette::{NamedSource, SourceSpan, Diagnostic};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Expected `(`")]
#[diagnostic(code(parser::error::ExpectedOpeningRoundParentheses))]
pub struct ExpectedOpeningRoundParentheses {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected `(` here"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected `)`")]
#[diagnostic(code(parser::error::ExpectedClosingRoundParentheses))]
pub struct ExpectedClosingRoundParentheses {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected `)` here"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected `{{` after {after}")]
#[diagnostic(code(parser::error::ExpectedOpeningCurlyParentheses))]
pub struct ExpectedOpeningCurlyParentheses {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected `{{` here"]
    pub(crate) err_span: SourceSpan,
    pub(crate) after: String,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected `}}`")]
#[diagnostic(code(parser::error::ExpectedClosingCurlyParentheses))]
pub struct ExpectedClosingCurlyParentheses {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected `}}` here"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected `else`after `if`")]
#[diagnostic(code(parser::error::ExpectedElse))]
pub struct ExpectedElse {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected `else` here"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected argument name")]
#[diagnostic(code(parser::error::ExpectedArgumentName))]
pub struct ExpectedArgumentName {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected argument name here"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected argument type")]
#[diagnostic(code(parser::error::ExpectedArgumentType))]
pub struct ExpectedArgumentType {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected argument type here"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected `:` between name and type")]
#[diagnostic(code(parser::error::ExpectedColonBetweenNameAndType))]
pub struct ExpectedColonBetweenNameAndType {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected `:` here"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected `;`")]
#[diagnostic(code(parser::error::ExpectedSemicolon))]
pub struct ExpectedSemicolon {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected `;` here"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected expression")]
#[diagnostic(
    code(parser::error::ExpectedExpression),
    help("{help_msg}")
)]
pub struct ExpectedExpression {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected expression here"]
    pub(crate) err_span: SourceSpan,
    pub(crate) help_msg: String,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Expected identifier")]
#[diagnostic(code(parser::error::ExpectedIdentifier))]
pub struct ExpectedIdentifier {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Expected identifier here"]
    pub(crate) err_span: SourceSpan,
}