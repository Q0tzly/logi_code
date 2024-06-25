# Lexer - Design
## Overview
- LogiCodeのLexerは、ソースコードを解析して、トークンに分解する役割を持つ。
- トークン列はその後のパース処理で利用される。
- 対話型のインタプリタでも利用できるように、tokenizeは一行づつ行われる。

## Purpose
- ソースコードの文字列を解析して、`TokenType`をつけて、`AST`に変換する処理をしやすくする。
- エラーを報告し、トークン化を中断しないようにする。

## Thinks
- バインドするとき以外はoutしか最初にくる文字がない。
つまり、outじゃない時は全てバインドになる。
バインド名のあとは、引数か区切り文字。

## TokenType
` rust
enum TokenType {
  Keyword, // 'input', 'out'
  Operator, // 'not', 'or'
  Identifier, // 'var', 'fn'
  Delimiter // ':'
  Literal, //'1', '0'
  Comment, // '//'
  Error,
}
`

## Struct
` rust
struct Position {
  line: u32,
  column: u32,
}

struct Token {
  token_type: TokenType,
  token_data: String,
}

struct Tokenizer {
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
