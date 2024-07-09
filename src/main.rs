use std::process::exit;

use logi_code::new;
use logi_code::{Evaluator, Parser, Tokenizer};

fn main() {
    let inputs = new();
    let mut tokenizer = Tokenizer::new(inputs);
    tokenizer.tokenize();
    let tokens = tokenizer.get_tokens();

    let mut parser = Parser::new(tokens.clone());
    parser.parse();

    let errors = parser.get_error();
    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}", error);
        }
        exit(0);
    }
    let ast = parser.get_ast();

    let mut evaluator = Evaluator::new(ast.clone());
    evaluator.evaluate();
}
