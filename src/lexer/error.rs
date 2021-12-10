use std::sync::Arc;

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Unknown symbol")]
#[diagnostic(code(lexer::error::UnknownSymbol))]
pub struct UnknownSymbol {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Unknown symbol"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Illegal symbol after non-equal operator starting with `=/`")]
#[diagnostic(
code(lexer::error::IllegalNonEqualToken),
help("Add `/` after `=/` to complete non-equal operator")
)]
pub struct IllegalNonEqualToken {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Illegal symbol"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Could not parse number")]
#[diagnostic(code(lexer::error::ParseNumberError))]
pub struct ParseNumberError {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Illegal number"]
    pub(crate) err_span: SourceSpan,
    // TODO: Show original parsing error
    // #[related]
    // pub others: Vec<miette::Report>,
}
