# Lexer - Design
## Overview
- LogiCodeのLexerは、ソースコードを解析して、トークンに分解する役割を持つ。
- トークン列はその後のパース処理で利用される。
- 対話型のインタプリタでも利用できるように、tokenizeは一行づつ行われる。

## Purpose
- ソースコードの文字列を解析して、`TokenType`をつけて、`AST`に変換する処理をしやすくする。
- エラーを報告し、トークン化を中断しないようにする。

## TokenType
` rust
enum TokenType {
  Keyword::Input,
  Keyword::Out,
  Identifier::Var,
  Identifier::Fn,
  Operator::Not,
  Operator::Or,
  Comment
}
`

## Struct
` rust
enum Position {
  line: u32,
  column: u32,
}

enum Tokenizer {
  token_type: TokenType,
  token_data: String,
}

enum Token {
 inputs: Vec<String>,
 tokens: Vec<Token>,
 position: Position,
}
`

## WorkFrow
input: Vec<String> (行ごとに文字列を分けておく。)
tokenizer = Tokenize::new(input)
tokenizer.tokenize()
tokens: Vec<Token> = tokenizer.get_tokens()
