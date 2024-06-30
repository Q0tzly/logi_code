use logi_code::{Evaluator, Parser, Tokenizer};

fn main() {
    let inputs: Vec<String> = vec![
        "input : A".to_string(),
        "B : 1".to_string(),
        "and A B : not or not A not B".to_string(),
        "C : and A B".to_string(),
        "out : A B C".to_string(),
    ];

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
