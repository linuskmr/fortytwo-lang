use std::collections::{HashMap, HashSet};

use crate::ast;
use crate::ast::AstNode;
use crate::position_container::PositionRangeContainer;

pub struct Runtime {
    variables: HashMap<String, f64>,
    functions: HashSet<ast::Function>,
    stack: Vec<HashSet<String>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashSet::new(),
            stack: vec![HashSet::new()],
        }
    }

    pub fn with_capacity(variables_cap: usize, functions_cap: usize, stack_cap: usize) -> Self {
        Self {
            variables: HashMap::with_capacity(variables_cap),
            functions: HashSet::with_capacity(functions_cap),
            stack: Vec::with_capacity(stack_cap),
        }
    }

    fn start_scope(&mut self) {
        self.stack.push(HashSet::new());
    }

    fn end_scope(&mut self) {
        let deleted_variables = match self.stack.pop() {
            Some(del_vars) => del_vars,
            None => return,
        };
        // Delete all variables from the current scope
        for deleted_var in deleted_variables {
            // TODO:
            self.variables.remove(&deleted_var);
        }
    }

    fn set_var(&mut self, name: String, value: f64) {
        if !self.variables.contains_key(&name) {
            // Associate variable to current scope
            self.stack.last_mut().unwrap().insert(name.clone());
        }
        // Update variable's content
        self.variables.insert(name, value);
    }

    fn get_var(&self, name: &str) -> Option<f64> {
        self.variables.get(name).cloned()
    }

    fn execute_binary_expression(&mut self, binop: ast::BinaryExpression) -> f64 {
        match binop.operator {
            ast::BinaryOperator::Addition => self.execute_ast(binop.lhs) + self.execute_ast(binop.rhs),
            ast::BinaryOperator::Multiplication => self.execute_ast(binop.lhs) * self.execute_ast(binop.rhs),
            ast::BinaryOperator::Subtraction => self.execute_ast(binop.lhs) - self.execute_ast(binop.rhs),
            ast::BinaryOperator::Less => {
                if self.execute_ast(binop.lhs) < self.execute_ast(binop.rhs) { 1.0 } else { 0.0 }
            }
        }
    }

    fn execute_function(&mut self, func: ast::Function) -> f64 {
        self.start_scope();
        let ret = self.execute_ast(func.body);
        self.end_scope();
        ret
    }

    fn execute_function_call(&mut self, func_call: ast::FunctionCall) -> f64 {
        self.start_scope();
        /*self.functions.get(func_call.name);
        // Push args onto the stack
        for arg in func_call.args.iter().zip() {
            self.set_var(self.execute_Ast(arg));
        }*/
        self.end_scope();
        0.0
    }

    pub fn execute_ast(&mut self, Ast: Box<ast::AstNode>) -> f64 {
        match *Ast {
            AstNode::BinaryExpression(binop) => self.execute_binary_expression(binop),
            AstNode::Number(PositionRangeContainer{data: num, ..}) => num,
            AstNode::Function(func) => self.execute_function(func),
            AstNode::FunctionCall(func_call) => self.execute_function_call(func_call),
            _ => 0.0,
        }
    }
}