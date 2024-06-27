enum ASTNode {
    Statement(Statement),
    Expression(Expression),
}

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

enum Expression {
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
        input: Vec<Identifier>,
    },
}

struct Parser {
    inputs: Vec<Token>,
    asts: Vec<ASTNode>,
}

impl Parser {
    fn new(input: Vec<Token>) -> Self {
        Self {
            inputs: input,
            ast: vec![],
        }
    }

    fn parse(&mut self) {}
}

#[cfg[test]]

mod tests {
    #[test]
    fn test_parser() {
        let tokens = [
            Token {
                token_type: Keyword,
                token_data: "input",
                token_line: 0,
            },
            Token {
                token_type: Delimiter,
                token_data: ":",
                token_line: 0,
            },
            Token {
                token_type: Identifier,
                token_data: "A",
                token_line: 0,
            },
            Token {
                token_type: Identifier,
                token_data: "B",
                token_line: 1,
            },
            Token {
                token_type: Delimiter,
                token_data: ":",
                token_line: 1,
            },
            Token {
                token_type: Literal,
                token_data: "1",
                token_line: 1,
            },
            Token {
                token_type: Identifier,
                token_data: "C",
                token_line: 2,
            },
            Token {
                token_type: Delimiter,
                token_data: ":",
                token_line: 2,
            },
            Token {
                token_type: Operator,
                token_data: "or",
                token_line: 2,
            },
            Token {
                token_type: Identifier,
                token_data: "A",
                token_line: 2,
            },
            Token {
                token_type: Identifier,
                token_data: "B",
                token_line: 2,
            },
            Token {
                token_type: Keyword,
                token_data: "out",
                token_line: 3,
            },
            Token {
                token_type: Identifier,
                token_data: "C",
                token_line: 3,
            },
        ];
    }
}
