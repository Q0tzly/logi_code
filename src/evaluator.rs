use core::panic;

use crate::parser::ASTNode;
use crate::parser::Expression;
use crate::parser::Statement;
use crate::utils::{stdin, stdout};

#[derive(Debug)]
pub struct VarInfo {
    pub name: String,
    pub value: bool,
}

#[derive(Clone, Debug)]
struct FnInfo {
    name: String,
    inputs: Vec<String>,
    expression: Expression,
}

#[derive(Debug)]
pub struct Evaluator {
    inputs: Vec<ASTNode>,
    pub var_list: Vec<VarInfo>,
    fn_list: Vec<FnInfo>,
}

impl Evaluator {
    pub fn new(ast: Vec<ASTNode>) -> Self {
        Self {
            inputs: ast,
            var_list: vec![],
            fn_list: vec![],
        }
    }

    pub fn evaluate(&mut self) {
        for node in &self.inputs.clone() {
            self.evaluate_statement(node);
        }
    }

    fn evaluate_statement(&mut self, node: &ASTNode) {
        if let ASTNode::Statement(statement) = node {
            match statement {
                Statement::BindVariable { name, expression } => {
                    let value = self.evaluate_expression(expression);
                    self.var_list.push(VarInfo {
                        name: name.clone(),
                        value,
                    });
                }
                Statement::BindFunction {
                    name,
                    input,
                    expression,
                } => self.fn_list.push(FnInfo {
                    name: name.clone(),
                    inputs: input.clone(),
                    expression: *expression.clone(),
                }),
                Statement::Input(inputs) => {
                    let values = stdin(inputs);
                    for (name, value) in inputs.iter().zip(values.into_iter()) {
                        self.var_list.push(VarInfo {
                            name: name.clone(),
                            value,
                        });
                    }
                }
                Statement::Output(outputs) => {
                    stdout(&self, outputs);
                }
            }
        }
    }

    fn evaluate_expression(&mut self, expression: &Expression) -> bool {
        match expression {
            Expression::Literal(value) => *value,
            Expression::Identifier(name) => {
                if let Some(var_info) = self.var_list.iter().find(|var| &var.name == name) {
                    var_info.value
                } else {
                    panic!("Variable {} not found in var_list", name);
                }
            }
            Expression::NOT { operand } => !self.evaluate_expression(&*operand),
            Expression::OR { left, right } => {
                self.evaluate_expression(&*left) || self.evaluate_expression(&*right)
            }
            Expression::Call { name, input } => {
                let current_fn_list = self.fn_list.clone();

                if let Some(fn_info) = current_fn_list.iter().find(|fn_info| &fn_info.name == name)
                {
                    let mut evaluated_args = Vec::new();
                    for expr in input {
                        let arg_value = match expr {
                            Expression::Identifier(ident_name) => {
                                if let Some(var_info) =
                                    self.var_list.iter().find(|var| &var.name == ident_name)
                                {
                                    var_info.value
                                } else {
                                    panic!("Variable {} not found in var_list", ident_name);
                                }
                            }
                            _ => self.evaluate_expression(expr),
                        };
                        evaluated_args.push(arg_value);
                    }

                    if evaluated_args.len() != fn_info.inputs.len() {
                        panic!("Mismatched number of arguments for function {}", name);
                    }

                    let mut temp_evaluator = Evaluator {
                        inputs: vec![],
                        var_list: Vec::new(),
                        fn_list: self.fn_list.clone(),
                    };

                    for (input_name, input_value) in fn_info.inputs.iter().zip(evaluated_args) {
                        temp_evaluator.var_list.push(VarInfo {
                            name: input_name.clone(),
                            value: input_value,
                        });
                    }

                    temp_evaluator.evaluate_expression(&fn_info.expression)
                } else {
                    panic!("Function {} not found in fn_list", name);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::evaluator::Evaluator;
    use crate::parser::{ASTNode, Expression, Statement};

    #[test]
    fn test_evaluator() {
        let ast: Vec<ASTNode> = vec![
            ASTNode::Statement(Statement::Input(vec!["A".to_string(), "B".to_string()])),
            ASTNode::Statement(Statement::BindFunction {
                name: String::from("and"),
                input: vec![String::from("A"), String::from("B")],
                expression: Box::new(Expression::NOT {
                    operand: Box::new(Expression::OR {
                        left: Box::new(Expression::NOT {
                            operand: Box::new(Expression::Identifier(String::from("A"))),
                        }),
                        right: Box::new(Expression::NOT {
                            operand: Box::new(Expression::Identifier(String::from("B"))),
                        }),
                    }),
                }),
            }),
            ASTNode::Statement(Statement::BindVariable {
                name: String::from("B"),
                expression: Box::new(Expression::Literal(true)),
            }),
            ASTNode::Statement(Statement::BindVariable {
                name: String::from("C"),
                expression: Box::new(Expression::Call {
                    name: String::from("and"),
                    input: vec![
                        Expression::Call {
                            name: String::from("and"),
                            input: vec![
                                Expression::Identifier(String::from("A")),
                                Expression::NOT {
                                    operand: Box::new(Expression::Identifier(String::from("B"))),
                                },
                            ],
                        },
                        Expression::Literal(false),
                    ],
                }),
            }),
            ASTNode::Statement(Statement::Output(vec![])),
        ];

        let mut evaluator = Evaluator::new(ast);
        evaluator.evaluate();
    }
}
