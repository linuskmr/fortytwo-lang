use crate::AST;

#[derive(Debug)]
pub struct Number(pub f64);

impl AST for Number {
    fn pretty(&self) -> String {
        self.0.to_string()
    }
}