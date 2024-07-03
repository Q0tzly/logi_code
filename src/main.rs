use logi_code::new;
use logi_code::{Evaluator, Parser, Tokenizer};

fn main() {
    let inputs = new();
    let mut tokenizer = Tokenizer::new(inputs);
    tokenizer.tokenize();
    let tokens = tokenizer.get_tokens();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens.clone());
    parser.parse();
    let ast = parser.get_ast();
    println!("{:?}", ast);

    let mut evaluator = Evaluator::new(ast.clone());
    evaluator.evaluate();
}
