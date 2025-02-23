use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    Literal(bool),
    NOT {
        operand: Box<Expression>,
    },
    OR {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Call {
        name: String,
        input: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    BindVariable {
        name: String,
        expression: Box<Expression>,
    },
    BindFunction {
        name: String,
        input: Vec<String>,
        expression: Box<Expression>,
    },
    Input(Vec<String>),
    Output(Vec<String>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Fn {
    name: String,
    input: u32,
}

#[derive(Debug)]
pub struct Position {
    line: u32,
}

#[derive(Debug)]
pub struct Parser {
    inputs: Vec<Vec<Token>>,
    ast: Vec<ASTNode>,
    error: Vec<String>,
    fn_list: Vec<Fn>,
    position: Position,
}

impl Parser {
    pub fn new(input: Vec<Vec<Token>>) -> Self {
        Self {
            inputs: input,
            ast: vec![],
            error: vec![],
            fn_list: vec![],
            position: Position { line: 0 },
        }
    }

    pub fn parse(&mut self) {
        for line in self.inputs.clone() {
            self.position.line += 1;

            if line.is_empty() {
                continue;
            }

            let result = &self.parse_statement(&line);
            match result {
                Ok(node) => {
                    self.ast.push(node.clone());
                }
                Err(e) => self
                    .error
                    .push(format!("{}: {}", self.position.line, e.to_string(),)),
            }
        }
    }

    fn parse_statement(&mut self, input: &[Token]) -> Result<ASTNode, String> {
        let mut iter = input.iter().peekable();
        if let Some(Token::Keyword(keyword)) = iter.peek() {
            match keyword.as_str() {
                "input" => {
                    iter.next();
                    if Some(&Token::Delimiter(":".to_string())) == iter.next() {
                        if let None = iter.peek() {
                            return Err("After the `:` comes an identifier that starts with a capital letter.".to_string());
                        }
                        let mut inputs = vec![];
                        let mut iter_all = iter.clone().cloned();

                        if !iter_all.all(|token| matches!(token, Token::Identifier(_))) {
                            return Err("Expected all tokens to be identifiers".to_string());
                        }
                        while let Some(Token::Identifier(name)) = iter.next() {
                            inputs.push(name.clone());
                        }
                        return Ok(ASTNode::Statement(Statement::Input(inputs)));
                    } else {
                        return Err("Expected `:` after `input`.".to_string());
                    }
                }
                "out" => {
                    iter.next();
                    if Some(&Token::Delimiter(":".to_string())) == iter.next() {
                        if let None = iter.peek() {
                            return Err("After the `:` comes an identifier that starts with a capital letter.".to_string());
                        }
                        let mut outputs = vec![];
                        let mut iter_all = iter.clone().cloned();
                        if !iter_all.all(|token| matches!(token, Token::Identifier(_))) {
                            return Err("Expected all tokens to be identifiers.".to_string());
                        }
                        while let Some(Token::Identifier(name)) = iter.next() {
                            outputs.push(name.clone());
                        }
                        return Ok(ASTNode::Statement(Statement::Output(outputs)));
                    } else {
                        return Err("Expected `:` after `out`".to_string());
                    }
                }
                _ => return Err("This Keyword does not exist.".to_string()),
            }
        }

        if let Some(Token::Identifier(name)) = iter.next() {
            if name.chars().next().unwrap().is_uppercase() {
                if Some(&Token::Delimiter(":".to_string())) == iter.peek().cloned() {
                    iter.next();
                    if let Some(expression) = self.parse_expression(&mut iter) {
                        if iter.peek().is_none() {
                            return Ok(ASTNode::Statement(Statement::BindVariable {
                                name: name.clone(),
                                expression: Box::new(expression),
                            }));
                        }
                    }
                    return Err("`:` is followed by an expression or a literal.".to_string());
                } else {
                    return Err("`:` is expected after the identifier.".to_string());
                }
            } else if name.chars().next().unwrap().is_lowercase() {
                if let Some(Token::Identifier(_)) = iter.peek() {
                    let mut inputs = vec![];
                    let mut list: u32 = 0;
                    while let Some(Token::Identifier(name)) = iter.peek() {
                        inputs.push(name.clone());
                        iter.next();
                        list += 1;
                    }
                    if Some(&Token::Delimiter(":".to_string())) == iter.next() {
                        if let Some(expression) = self.parse_expression(&mut iter) {
                            if iter.peek().is_none() {
                                let _ = &self.fn_list.push(Fn {
                                    name: name.clone(),
                                    input: list,
                                });
                                return Ok(ASTNode::Statement(Statement::BindFunction {
                                    name: name.clone(),
                                    input: inputs,
                                    expression: Box::new(expression),
                                }));
                            }
                        } else {
                            return Err("Expected expression or literal after the `:`.".to_string());
                        }
                    } else {
                        return Err("`:` come after the indentifier.".to_string());
                    }
                }
                return Err("When binding in lower case, the identifier comes after.".to_string());
            }
        }

        Err("This syntax does not exist.".to_string())
    }

    fn parse_expression<'a, I>(&mut self, tokens: &mut std::iter::Peekable<I>) -> Option<Expression>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token = tokens.next()?;
        match token {
            Token::Operator(op) if op == "not" => {
                let operand = self.parse_expression(tokens)?;
                Some(Expression::NOT {
                    operand: Box::new(operand),
                })
            }
            Token::Operator(op) if op == "or" => {
                let left = &self.parse_expression(tokens)?;
                let right = &self.parse_expression(tokens)?;
                Some(Expression::OR {
                    left: Box::new(left.clone()),
                    right: Box::new(right.clone()),
                })
            }
            Token::Identifier(name) if name.chars().next().unwrap().is_uppercase() => {
                Some(Expression::Identifier(name.clone()))
            }
            Token::Identifier(fn_name) => {
                if let Some(fn_info) = self.fn_list.iter().find(|f| &f.name == fn_name) {
                    let mut inputs: Vec<Expression> = vec![];
                    for _ in 0..fn_info.input {
                        if let Some(expr) = self.parse_expression(tokens) {
                            inputs.push(expr);
                        } else {
                            return None;
                        }
                    }
                    Some(Expression::Call {
                        name: fn_name.to_string(),
                        input: inputs,
                    })
                } else {
                    //"Undefined bind name."
                    None
                }
            }
            Token::Literal(value) if value == "1" => Some(Expression::Literal(true)),
            Token::Literal(value) if value == "0" => Some(Expression::Literal(false)),
            _ => None,
        }
    }

    pub fn get_ast(&self) -> &Vec<ASTNode> {
        &self.ast
    }

    pub fn get_error(&self) -> &Vec<String> {
        &self.error
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use crate::parser::{ASTNode, Parser, Statement};
    #[test]
    fn test_parser() {
        let input: Vec<Vec<Token>> = vec![
            vec![
                Token::Keyword("input".to_string()),
                Token::Delimiter(":".to_string()),
                Token::Identifier("A".to_string()),
            ],
            vec![
                Token::Identifier("and".to_string()),
                Token::Identifier("A".to_string()),
                Token::Identifier("B".to_string()),
                Token::Delimiter(":".to_string()),
                Token::Operator("not".to_string()),
                Token::Operator("or".to_string()),
                Token::Operator("not".to_string()),
                Token::Identifier("A".to_string()),
                Token::Operator("not".to_string()),
                Token::Identifier("B".to_string()),
            ],
            vec![
                Token::Identifier("B".to_string()),
                Token::Delimiter(":".to_string()),
                Token::Literal("1".to_string()),
            ],
            vec![
                Token::Identifier("C".to_string()),
                Token::Delimiter(":".to_string()),
                Token::Identifier("and".to_string()),
                Token::Identifier("and".to_string()),
                Token::Identifier("A".to_string()),
                Token::Operator("not".to_string()),
                Token::Identifier("B".to_string()),
                Token::Literal("0".to_string()),
            ],
            vec![],
            vec![
                Token::Keyword("out".to_string()),
                Token::Delimiter(":".to_string()),
                Token::Identifier("C".to_string()),
            ],
            vec![Token::Error("HEllo".to_string())],
        ];
        //let input = vec![];
        let mut parser = Parser::new(input);
        parser.parse();
        let ast = parser.get_ast();
        let error = parser.get_error();
        println!("Error: {:?}", error);
        assert_eq!(
            ast,
            &vec![ASTNode::Statement(Statement::Input(vec!["B".to_string()]))]
        );
    }
}
