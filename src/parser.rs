use crate::lexer::Token;

#[derive(Debug, PartialEq)]
enum Expression {
    Identifier(String), // Variable
    Literal(bool),
    NOT {
        operand: Box<Expression>,
    },
    OR {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // Function
    Call {
        name: String,
        input: Vec<String>,
    },
}

#[derive(Debug, PartialEq)]
enum Statement {
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

#[derive(Debug, PartialEq)]
enum ASTNode {
    Statement(Statement),
}

#[derive(Debug)]
pub struct Parser {
    inputs: Vec<Vec<Token>>,
    ast: Vec<ASTNode>,
    error: Vec<String>,
}

impl Parser {
    pub fn new(input: Vec<Vec<Token>>) -> Self {
        Self {
            inputs: input,
            ast: vec![],
            error: vec![],
        }
    }

    pub fn parse(&mut self) {
        for line in &self.inputs {
            if line.is_empty() {
                continue;
            }
            match Self::parse_statement(line) {
                Ok(node) => self.ast.push(node),
                Err(e) => self.error.push(e),
            }
        }
    }

    fn parse_statement(input: &Vec<Token>) -> Result<ASTNode, String> {
        let mut iter = input.iter().peekable();
        if let Some(Token::Keyword(keyword)) = iter.peek() {
            match keyword.as_str() {
                "input" => {
                    iter.next();
                    if Some(&Token::Delimiter(":".to_string())) == iter.next() {
                        let mut inputs = vec![];
                        while let Some(Token::Identifier(name)) = iter.next() {
                            inputs.push(name.clone());
                        }
                        return Ok(ASTNode::Statement(Statement::Input(inputs)));
                    }
                }
                "out" => {
                    iter.next();
                    if Some(&Token::Delimiter(":".to_string())) == iter.next() {
                        let mut outputs = vec![];
                        while let Some(Token::Identifier(name)) = iter.next() {
                            outputs.push(name.clone());
                        }
                        return Ok(ASTNode::Statement(Statement::Output(outputs)));
                    }
                }
                _ => return Err("Error".to_string()),
            }
        }

        if let Some(Token::Identifier(name)) = iter.next() {
            if name.chars().next().unwrap().is_uppercase()
                && Some(&Token::Delimiter(":".to_string())) == iter.next()
            {
                if let Some(expression) = Self::parse_expression(&mut iter) {
                    if iter.peek().is_none() {
                        return Ok(ASTNode::Statement(Statement::BindVariable {
                            name: name.clone(),
                            expression: Box::new(expression),
                        }));
                    }
                }
            } else {
                let mut inputs = vec![];
                while let Some(Token::Identifier(name)) = iter.next() {
                    inputs.push(name.clone());
                }
                if Some(&Token::Delimiter(":".to_string())) == iter.next() {
                    if let Some(expression) = Self::parse_expression(&mut iter) {
                        if iter.peek().is_none() {
                            return Ok(ASTNode::Statement(Statement::BindFunction {
                                name: name.clone(),
                                input: inputs,
                                expression: Box::new(expression),
                            }));
                        }
                    }
                }
            }
        }
        Err("Unable to parse line".to_string())
    }

    fn parse_expression<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Option<Expression>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token = tokens.next()?;
        match token {
            Token::Identifier(name) => Some(Expression::Identifier(name.clone())),
            Token::Literal(value) if value == "1" => Some(Expression::Literal(true)),
            Token::Literal(value) if value == "0" => Some(Expression::Literal(false)),
            Token::Operator(op) if op == "not" => {
                let operand = Self::parse_expression(tokens)?;
                Some(Expression::NOT {
                    operand: Box::new(operand),
                })
            }
            Token::Operator(op) if op == "or" => {
                let left = Self::parse_expression(tokens)?;
                let right = Self::parse_expression(tokens)?;
                Some(Expression::OR {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
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
            //clear
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
            //clear
            vec![
                Token::Identifier("B".to_string()),
                Token::Delimiter(":".to_string()),
                Token::Literal("1".to_string()),
            ],
            vec![
                Token::Identifier("C".to_string()),
                Token::Delimiter(":".to_string()),
                Token::Identifier("and".to_string()),
                Token::Identifier("A".to_string()),
                Token::Identifier("B".to_string()),
            ],
            //clear
            vec![],
            //clear
            vec![
                Token::Keyword("out".to_string()),
                Token::Delimiter(":".to_string()),
                Token::Identifier("C".to_string()),
            ],
        ];
        let mut parser = Parser::new(input);
        parser.parse();
        let ast = parser.get_ast();
        assert_eq!(
            ast,
            &vec![ASTNode::Statement(Statement::Input(vec!["B".to_string()]))]
        );
    }
}
