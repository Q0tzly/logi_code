# Parser - Design
## Overview
- LogiCodeのParserはトークン列を解析して、ASTを生成する役割を持つ。
- ASTはその後の実行処理で利用される。
- トークン列1行ごとにASTに変換される。(初期リリース後のマイナーバージョンアップでlexerの方の出力をvec<vec<Token>>に変更する。)

## Purpose
- ASTのTypeをいくつか定義して、実行処理をしやすくする。
- コメントをここで取り除く。

## AstType
` rust
enum AstType {
  Bind, //Identifier, Args, Expression
  Call, //Identifier, Args
  Args(Vec<Identifier>), //[Identifier(String), Identifier(String), ..]
  Expression,
  Or,
  Not,
  Literal(bool),
  Identifier(String),
}
`

## Struct
` rust
struct Ast {
  ast_type: AstType,
  right: Ast,
  left: Ast,
}

struct Parser {
  inuputs: Vec<Token>,
  asts: Vec<Ast>,
}
`

## WorkFrow
input: Vec<Token>
parser = Parser::new(input)
parser.parse()
ast: Vec<AST> = parser.get_ast()

## AstExamples
`
  [AST { ast_type: Bind, right: AST { ast_type: Identifier, right: }]
`
