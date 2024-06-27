#[derive(Debug, PartialEq)]
enum TokenType {
    Keyword,
    Operator,
    Identifier,
    Delimiter,
    Literal,
    Comment,
    Error,
}

struct Position {
    line: u32,
    column: u32,
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    token_data: String,
}

pub struct Tokenizer {
    inputs: Vec<String>,
    tokens: Vec<Vec<Token>>,
    position: Position,
}

impl Tokenizer {
    pub fn new(input: Vec<String>) -> Self {
        Self {
            inputs: input,
            tokens: vec![],
            position: Position { line: 0, column: 0 },
        }
    }

    pub fn tokenize(&mut self) {
        for line in &self.inputs {
            let mut token = vec![];
            if Self::is_comment(&line) {
                let _ = token.push(Token {
                    token_type: TokenType::Comment,
                    token_data: line.clone(),
                });
            } else {
                let columns = line.split_whitespace();

                for column in columns {
                    let t_type = if Self::is_keyword(&column) {
                        TokenType::Keyword
                    } else if Self::is_operator(&column) {
                        TokenType::Operator
                    } else if Self::is_identifier(&column) {
                        TokenType::Identifier
                    } else if Self::is_delimiter(&column) {
                        TokenType::Delimiter
                    } else if Self::is_literal(&column) {
                        TokenType::Literal
                    } else {
                        TokenType::Error
                    };

                    let _ = token.push(Token {
                        token_type: t_type,
                        token_data: column.to_string(),
                    });
                    self.position.column += 1;
                }
            };
            self.position.line += 1;
            let _ = &self.tokens.push(token);
        }
    }

    fn get_tokens(&self) -> &Vec<Vec<Token>> {
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
        input.starts_with("//")
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Token, Tokenizer};

    #[test]
    fn test_lexer() {
        let input: Vec<String> = vec![
            "input : A\n".to_string(),
            "B : 1\n".to_string(),
            "C : or A B\n".to_string(),
            "out C".to_string(),
        ];
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        let tokens = tokenizer.get_tokens();
        assert_eq!(
            tokens,
            &vec![vec![Token {
                token_type: crate::lexer::TokenType::Comment,
                token_data: "// Hello".to_string(),
            },]]
        );
    }
}
