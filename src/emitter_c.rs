use crate::ast::{
    AstNode, BasicDataType, BinaryExpression, BinaryOperator, DataType, Expression, Function,
    FunctionArgument, FunctionCall, FunctionPrototype, IfExpression, Statement,
};
use crate::position_container::PositionRangeContainer;
use std::io;
use std::io::{BufWriter, Write};

/// EmitterC reads [AstNode]s and generates c sourcecode from it.
pub struct EmitterC<Writer: Write> {
    target: BufWriter<Writer>,
}

impl<Writer: Write> EmitterC<Writer> {
    /// Create a new [EmitterC] from the given [AstNode] iterator.
    pub fn codegen<AstIter>(source_ast_nodes: AstIter, target: BufWriter<Writer>) -> io::Result<()>
    where
        AstIter: Iterator<Item = AstNode>,
    {
        let mut emitter = Self { target };
        source_ast_nodes
            .into_iter()
            .try_for_each(|ast_node| emitter.codegen_ast_node(ast_node))?;
        emitter.target.flush()
    }

    fn codegen_ast_node(&mut self, ast_node: AstNode) -> io::Result<()> {
        match ast_node {
            AstNode::Expression(expression) => self.expression(expression)?,
            AstNode::Statement(statement) => self.statement(statement)?,
        }
        self.write("\n")
    }

    fn expression(&mut self, expression: Expression) -> io::Result<()> {
        match expression {
            Expression::BinaryExpression(binary_expression) => {
                self.binary_expression(binary_expression)
            }
            Expression::FunctionCall(function_call) => self.function_call(function_call),
            Expression::Number(number) => self.number(number),
            Expression::Variable(variable) => self.variable(variable),
            Expression::IfExpression(if_expression) => self.if_expression(*if_expression),
        }
    }

    /// Generates code for an [IfExpression].
    fn if_expression(&mut self, if_expression: IfExpression) -> io::Result<()> {
        self.write("(")?;
        self.expression(if_expression.condition)?;
        self.write("? ")?;
        self.expression(if_expression.if_true)?;
        self.write(" : ")?;
        self.expression(if_expression.if_false)?;
        self.write(")")
    }

    /// Generates code for a [BinaryExpression].
    fn binary_expression(&mut self, binary_expression: BinaryExpression) -> io::Result<()> {
        self.write("(")?;
        self.expression(*binary_expression.lhs)?;
        self.write(" ")?;
        self.binary_operator(binary_expression.operator)?;
        self.write(" ")?;
        self.expression(*binary_expression.rhs)?;
        self.write(")")
    }

    /// Generates code for a [BinaryOperator].
    fn binary_operator(
        &mut self,
        binary_operator: PositionRangeContainer<BinaryOperator>,
    ) -> io::Result<()> {
        self.write(match binary_operator.data {
            BinaryOperator::Less => "<",
            BinaryOperator::Greater => ">",
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
        })
    }

    /// Generates code for a [FunctionCall].
    fn function_call(&mut self, function_call: FunctionCall) -> std::io::Result<()> {
        self.write(&function_call.name.data)?;
        self.write("(")?;
        self.function_call_params(function_call.params)?;
        self.write(")")
    }

    /// Generates code for the parameters of a [FunctionCall].
    fn function_call_params(&mut self, params: Vec<Expression>) -> std::io::Result<()> {
        let mut params = params.into_iter().peekable();
        while let Some(param) = params.next() {
            // Generate code for each param
            self.expression(param)?;
            // Only write a `,` if this is not the last param
            if params.peek().is_some() {
                self.write(", ")?;
            }
        }
        Ok(())
    }

    /// Generates code for a number.
    fn number(&mut self, number: PositionRangeContainer<f64>) -> std::io::Result<()> {
        self.write(&number.data.to_string())
    }

    /// Generates code for a variable.
    fn variable(&mut self, variable: PositionRangeContainer<String>) -> std::io::Result<()> {
        self.write(&variable.data)
    }

    /// Generates code for a statement.
    fn statement(&mut self, statement: Statement) -> std::io::Result<()> {
        match statement {
            Statement::FunctionPrototype(function_prototype) => {
                self.function_prototype(function_prototype)?;
                self.write(";\n") // End function header with semicolon
            }
            Statement::Function(function) => self.function(function),
        }
    }

    /// Generates code for a function prototype.
    fn function_prototype(&mut self, function_prototype: FunctionPrototype) -> std::io::Result<()> {
        // TODO: Change constant return type double to appropriate type
        self.write("double ")?;
        self.write(&function_prototype.name.data)?;
        self.write("(")?;
        self.function_prototype_args(function_prototype.args)?;
        self.write(")")
    }

    /// Generates code for the [FunctionArgument] of a [FunctionPrototype].
    fn function_prototype_args(&mut self, args: Vec<FunctionArgument>) -> std::io::Result<()> {
        let mut args = args.into_iter().peekable();
        while let Some(arg) = args.next() {
            // Generate code for each param
            self.function_argument(arg)?;
            // Only write a `,` if this is not the last param
            if args.peek().is_some() {
                self.write(", ")?;
            }
        }
        Ok(())
    }

    /// Generates code for a [FunctionArgument].
    fn function_argument(&mut self, arg: FunctionArgument) -> std::io::Result<()> {
        self.data_type(arg.data_type)?;
        self.write(" ")?;
        self.write(&arg.name.data)
    }

    /// Generates code for a [DataType].
    fn data_type(&mut self, data_type: PositionRangeContainer<DataType>) -> std::io::Result<()> {
        match data_type.data {
            DataType::Basic(basic_data_type) => self.basic_data_type(basic_data_type),
            DataType::Struct(struct_name) => self.write(&struct_name),
            DataType::Pointer(ptr) => {
                self.data_type(*ptr)?;
                self.write("*")
            }
        }
    }

    /// Generates code for [BasicDataType].
    fn basic_data_type(&mut self, basic_data_type: BasicDataType) -> io::Result<()> {
        self.write(match basic_data_type {
            BasicDataType::Int => "int",
            BasicDataType::Float => "double",
        })
    }

    /// Generates for a [Function].
    fn function(&mut self, function: Function) -> io::Result<()> {
        self.function_prototype(function.prototype)?;
        self.write(" {\nreturn ")?;
        self.expression(function.body)?;
        self.write(";\n}\n")
    }

    /// Writes `text` into [EmitterC::target].
    fn write(&mut self, mut text: &str) -> io::Result<()> {
        self.target.write(text.as_bytes())?;
        Ok(())
    }
}

/// Executes `foreach` on each element in `iter` and executes `separator` between adjacent items of `iter`.
fn foreach_intersperse<T>(
    iter: impl Iterator<Item = T>,
    for_each: impl Fn(T),
    separator: impl Fn(),
) {
    let mut iter = iter.peekable();
    for element in iter.next() {
        for_each(element);
        if iter.peek().is_some() {
            separator()
        }
    }
}
