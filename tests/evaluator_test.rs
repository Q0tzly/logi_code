use logi_code::{Evaluator, Parser, Tokenizer};

#[test]
fn test_evaluator() {
    let inputs: Vec<String> = vec![
        "input : A B".to_string(),
        "and A B : not or not A not B".to_string(),
        "C : and A B".to_string(),
        "out : C".to_string(),
    ];

    let mut tokenizer = Tokenizer::new(inputs);
    tokenizer.tokenize();
    let tokens = tokenizer.get_tokens();

    let mut parser = Parser::new(tokens.clone());
    parser.parse();
    let ast = parser.get_ast();

    let mut evaluator = Evaluator::new(ast.clone());
    evaluator.evaluate();
}
