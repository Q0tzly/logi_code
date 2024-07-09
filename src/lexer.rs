#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Operator(String),
    Identifier(String),
    Delimiter(String),
    Literal(String),
    Error(String),
}

pub struct Tokenizer {
    inputs: Vec<String>,
    tokens: Vec<Vec<Token>>,
}

impl Tokenizer {
    pub fn new(input: Vec<String>) -> Self {
        Self {
            inputs: input,
            tokens: vec![],
        }
    }

    pub fn tokenize(&mut self) {
        for line in &self.inputs {
            let mut token = vec![];
            if Self::is_comment(line) {
                let _ = &self.tokens.push(vec![]);
                continue;
            } else {
                let columns = line.split_whitespace();

                for column in columns {
                    token.push(if Self::is_keyword(column) {
                        Token::Keyword(column.to_string())
                    } else if Self::is_operator(column) {
                        Token::Operator(column.to_string())
                    } else if Self::is_identifier(column) {
                        Token::Identifier(column.to_string())
                    } else if Self::is_delimiter(column) {
                        Token::Delimiter(column.to_string())
                    } else if Self::is_literal(column) {
                        Token::Literal(column.to_string())
                    } else {
                        Token::Error(column.to_string())
                    });
                }
            };
            let _ = &self.tokens.push(token);
        }
    }

    pub fn get_tokens(&self) -> &Vec<Vec<Token>> {
        &self.tokens
    }

    fn is_keyword(input: &str) -> bool {
        input == "input" || input == "out"
    }

    fn is_operator(input: &str) -> bool {
        input == "not" || input == "or"
    }

    fn is_identifier(input: &str) -> bool {
        input.chars().all(|c| c.is_ascii_alphabetic())
            && input[1..].chars().all(|c| c.is_ascii_lowercase())
    }

    fn is_delimiter(input: &str) -> bool {
        input == ":"
    }

    fn is_literal(input: &str) -> bool {
        input == "1" || input == "0"
    }

    fn is_comment(input: &str) -> bool {
        input.starts_with("//") || input.starts_with("#")
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Token, Tokenizer};

    #[test]
    fn test_lexer() {
        let input: Vec<String> = vec![
            "".to_string(),
            "input : A\n".to_string(),
            "and A B : not or not A not B".to_string(),
            "B : 1\n".to_string(),
            "C : and A B\n".to_string(),
            "".to_string(),
            "out : C".to_string(),
            "//".to_string(),
            "#".to_string(),
        ];

        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        let tokens = tokenizer.get_tokens();
        assert_eq!(tokens, &vec![vec![Token::Error("Hello".to_string()),]]);
    }
}
