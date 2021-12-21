
use std::sync::Arc;

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Syntax error")]
#[diagnostic(code(error::SyntaxError))]
pub struct SyntaxError {
    #[source_code]
    pub(crate) src: Arc<NamedSource>,
    #[label = "Got"]
    pub(crate) err_span: SourceSpan,
    pub(crate) expected: &'static str,
}