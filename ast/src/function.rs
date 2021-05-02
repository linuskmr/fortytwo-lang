use crate::AST;

#[derive(Debug)]
pub struct FunctionPrototype {
    name: String,
    args: Vec<String>,
}

impl AST for FunctionPrototype {
    fn pretty(&self) -> String {
        format!("{}{:?}", self.name, self.args)
    }
}

#[derive(Debug)]
pub struct Function {
    prototype: FunctionPrototype,
    body: dyn AST,
}

impl AST for Function {
    fn pretty(&self) -> String {
        format!("{}\n{}", self.prototype.pretty(), self.body.pretty())
    }
}