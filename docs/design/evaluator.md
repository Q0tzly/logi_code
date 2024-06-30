# Evaluator - Design
## Overview
- LogiCodeのEvaluatorはASTを解析して、評価する役割を持つ。
- 入出力はUtilsによって行われる。
- 評価はVec<ASTNode>を順に評価していく。

- シンタックスエラーは、Parserで全て取り除いておく。(インタプリタでもコンパイラでも使えるように。)

## Purpose
- インタプリタの評価する部分を分離して、コードを読みやすく、メンテナスしやすくする。

## Think
- callの引数に演算子が入っているとparserがエラーを吐く。

## Structs
` rust
struct VarInfo {
  name: String,
  value: bool,
}

struct FnInfo {
  name: String,
  input: String,
  expression: ASTNode::Expression,
}

struct Evaluator {
  inputs: Vec<ASTNode>,
  var_list: Vec<VarInfo>,
  fn_list: Vec<FnInfo>,
}
`
## WorkFlow
### Input
[
Statement(Input(["A"])),
Statement(BindFunction { name: "and", input: ["A", "B"], expression: NOT { operand: OR { left: NOT { operand: Identifier("A") }, right: NOT { operand: Identifier("B") } } } }),
Statement(BindVariable { name: "B", expression: Literal(true) }),
Statement(BindVariable { name: "C", expression: Call { name: "and", input: [Identifier("A"), NOT { operand: Identifier("B") }] } }),
Statement(Output(["C"]))
]

### evaluate()
- astを一行ごとに分割して、evalute_statement()を呼び出す。

### evalute_statement()
- match文でそれが何かを判断して、適宜処理する。
**BindVar**
- (name, expressionを評価した結果)を、var_listに突っ込む

**BindFn**
- (name, input, expression)を、fn_listに突っ込む

**Input**
- inputのVec<String>をutilsに渡して、Vec<bool>を得る。
- (Vec<String>, Vec<bool>)を順番にvar_listに突っ込む。

**Out**
- inputのVec<String>をutilsに渡して表示する。

### evaluate_expression()
- match文で適宜処理する。
**Literal**
- Expression::Literal(bool)のboolをそのまま返す。
**Identifier**
- Expression::Identifier(String)のStringをself.var_listから検索して、valueを持ってくる。
**NOT**
- Expression::NOTは、operandをevaluate_expresson()で評価したあと、bool値を反転させる。
**OR**
- Expression::ORは、left, rightをevaluate_expression()で評価した後、`right || left`の値を返す。
**Call**
- Expression::Callは、inputsをそれぞれevaluate_expression()で評価する。
- fn_listから、nameと一致するものを探す。見つからなかったら、エラーを吐く。
- 見つかったら、inputとinputsの順番を同じようにして、expressionに代入した後、それを評価する。

## Usage
` rust
input: Vec<ASTNode>
evaluator = Evaluator::new(input);
evaluator.evaluate();
`

` bash
> input: A: □, B: ■
> out: C: ■
`
