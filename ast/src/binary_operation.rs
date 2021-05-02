use crate::AST;

#[derive(Debug)]
struct BinaryOperation<L: AST, R: AST> {
    lhs: L,
    operation: char,
    rhs: R,
}

impl<L: AST, R: AST> AST for BinaryOperation<L, R> {
    fn pretty(&self) -> String {
        format!("{} {} {}", self.lhs.pretty(), self.operation, self.rhs.pretty())
    }
}