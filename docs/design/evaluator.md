# Evaluator - Design
## Overview
- LogiCodeのEvaluatorはASTを解析して、評価する役割を持つ。
- 入出力はUtilsによって行われる。
- 評価はVec<ASTNode>を順に評価していく。

- シンタックスエラーは、Parserで全て取り除いておく。(インタプリタでもコンパイラでも使えるように。)

## Purpose
- インタプリタの評価する部分を分離して、コードを読みやすく、メンテナスしやすくする。

## Structs
` rust
struct FnInfo {
  name: String,
  inputs: u32,
}

struct Evaluator {
  inputs: Vec<ASTNode>,
  fn_list: Vec<FnInfo>,
}
`
## WorkFlow

## Usage
` rust
input: Vec<ASTNode>
evaluator = Evaluator::new(&input);
evaluator.execute();
`

` bash
> input: A: □, B: ■
> out: C: ■
`
