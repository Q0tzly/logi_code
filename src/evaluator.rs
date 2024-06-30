use core::panic;

use crate::parser::ASTNode;
use crate::parser::Expression;
use crate::parser::Statement;
use crate::utils::std_input;

struct VarInfo {
    name: String,
    value: bool,
}

#[derive(Clone)]
struct FnInfo {
    name: String,
    inputs: Vec<String>,
    expression: Expression,
}

pub struct Evaluator {
    inputs: Vec<ASTNode>,
    var_list: Vec<VarInfo>,
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
        match node {
            ASTNode::Statement(statement) => match statement {
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
                    let values = std_input(inputs);
                    for (name, value) in inputs.iter().zip(values.into_iter()) {
                        self.var_list.push(VarInfo {
                            name: name.clone(),
                            value,
                        });
                    }
                }
                Statement::Output(outputs) => {
                    self.evaluate_output(outputs);
                }
            },
            _ => (),
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
                if let Some(fn_info) = self.fn_list.iter().find(|fn_info| &fn_info.name == name) {
                    let mut temp_evaluator = Evaluator {
                        inputs: vec![ASTNode::Expression(fn_info.expression.clone())],
                        var_list: Vec::new(),
                        fn_list: self.fn_list.clone(),
                    };

                    let evaluated_inputs: Vec<bool> = input
                        .iter()
                        .map(|expr| temp_evaluator.evaluate_expression(expr))
                        .collect();

                    if evaluated_inputs.len() != fn_info.inputs.len() {
                        panic!("Mismatched number of arguments for function {}", name);
                    }

                    for (var_name, var_value) in fn_info.inputs.iter().zip(evaluated_inputs.iter())
                    {
                        temp_evaluator.var_list.push(VarInfo {
                            name: var_name.clone(),
                            value: *var_value,
                        });
                    }

                    temp_evaluator.evaluate();

                    temp_evaluator.var_list.last().unwrap().value
                } else {
                    panic!("Function {} not found in fn_list", name);
                }
            }
        }
    }

    fn evaluate_output(&self, outputs: &[String]) {
        let mut index = 0;
        print!("out > ");
        for name in outputs {
            index += 1;
            for var_info in &self.var_list {
                if &var_info.name == name {
                    let value = match var_info.value {
                        true => "■".to_string(),
                        false => "□".to_string(),
                        _ => panic!("Unexpected literals"),
                    };
                    print!("{} {}", name, value);
                }
            }
            if index < outputs.len() {
                print!(" : ");
            }
        }
        println!();
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
