use std::collections::{HashMap, HashSet};

use crate::ast;
use crate::ast::AST;
use crate::position_container::PositionRangeContainer;

pub struct Runtime {
    variables: HashMap<String, f64>,
    stack: Vec<HashSet<String>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            stack: vec![HashSet::new()],
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

        for deleted_var in deleted_variables {
            self.variables.remove(&deleted_var);
        }
    }

    fn set_var(&mut self, name: String, value: f64) {
        if !self.variables.contains_key(&name) {
            // Associate var to current scope
            self.stack.last_mut().unwrap().insert(name.clone());
        }
        self.variables.insert(name, value);
    }

    fn get_var(&self, name: &str) -> Option<f64> {
        self.variables.get(name).cloned()
    }

    fn execute_binary_expression(&mut self, binop: ast::BinaryExpression) -> f64 {
        match binop.operator {
            ast::BinaryOperator::Plus => self.execute_ast(binop.lhs) + self.execute_ast(binop.rhs),
            ast::BinaryOperator::Times => self.execute_ast(binop.lhs) * self.execute_ast(binop.rhs),
            ast::BinaryOperator::Minus => self.execute_ast(binop.lhs) - self.execute_ast(binop.rhs),
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

    pub fn execute_ast(&mut self, ast: Box<ast::AST>) -> f64 {
        match *ast {
            AST::BinaryExpression(binop) => self.execute_binary_expression(binop),
            AST::Number(PositionRangeContainer{data: num, ..}) => num,
            AST::Function(func) => self.execute_function(func),
            _ => 0.0,
        }
    }
}