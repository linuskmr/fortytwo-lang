mod number;
mod variable;
mod binary_operation;
mod function_call;

use std::fmt::Debug;

trait AST: Debug {
    fn pretty(&self) -> String;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
