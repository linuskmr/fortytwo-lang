use crate::AST;

#[derive(Debug)]
pub struct Variable(pub String);

impl AST for Variable {
    fn pretty(&self) -> String {
        String::from(&self.0)
    }
}