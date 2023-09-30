//! Stages of the semantic analysis.

/// In this pass, the [`SemanticAnalyzer`](crate::semantic_analyzer::SemanticAnalyzer) will scan the code for global symbols, i.e. function definitions.
#[derive(Default, Debug)]
pub struct GlobalSymbolScan;

/// In this pass, the [`SemanticAnalyzer`](crate::semantic_analyzer::SemanticAnalyzer) will check that all variables are declared before they are used and that the types are correct.
#[derive(Default, Debug)]
pub struct TypeCheck;
