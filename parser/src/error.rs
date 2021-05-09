pub struct ParsingError {
    pub(crate) kind: ParsingErrorKind,
    pub(crate) message: String,
    pub(crate) position: Option<lexer::PositionRange>,
}

impl ParsingError {

    fn kind(&self) -> ParsingErrorKind {
        self.kind.clone()
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn position(&self) -> &Option<lexer::PositionRange> {
        &self.position
    }
}

#[derive(Clone)]
pub(crate) enum ParsingErrorKind {
    ExpectedExpression,
    ExpectedSymbol,
}