use crate::AST;

#[derive(Debug)]
pub(crate) struct Number(pub(crate) f64);

impl AST for Number {
    fn pretty(&self) -> String {
        self.0.to_string()
    }
}