use crate::AST;

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub args: Vec<Box<dyn AST>>,
}

impl AST for FunctionCall {
    fn pretty(&self) -> String {
        // TODO: Fix join: the following trait bounds were not satisfied: `<[Box<dyn AST>] as Join<_>>::Output = _`
        // format!("{}({:?})", self.function_name, self.args.join(", "))
        let mut args = String::with_capacity(self.args.len());
        for arg in &self.args {
            args += &format!("{}, ", arg.pretty());
        }
        format!("{}({})", self.function_name, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::{number::Number, variable::Variable};

    use super::*;

    #[test]
    fn pretty() {
        let function_call = FunctionCall {
            function_name: String::from("my_method"),
            args: vec![
                Box::new(Number(4.2)),
                Box::new(Variable(String::from("var")))
            ],
        };
        assert_eq!(&function_call.pretty(), "my_method(4.2, var, )");
    }
}