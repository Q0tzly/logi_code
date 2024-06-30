# Parser - Design
## Overview
- LogiCodeのParserはトークン列を解析して、ASTを生成する役割を持つ。
- ASTはその後の実行処理で利用される。
- トークン列1行ごとにASTに変換される。(初期リリース後のマイナーバージョンアップでlexerの方の出力をvec<vec<Token>>に変更する。)

## Purpose
- ASTのTypeをいくつか定義して、実行処理をしやすくする。
- コメントをここで取り除く。

## Things
- Bindする時、入力をとるバインドは必ず入力は大文字(値だから)
- また、この時、引数じゃない値は使えない。
- bindする時、入力を撮る時も取らない時も、外部の関数は使える。
- 入力なしのバインドの時は、外部の変数も使える。
- 何も書かれてない行(改行文字とスペースはあってもいい)の時、トークンは何もない`[]`を吐き出すので、それの対処も必要。

## Fix
- Callの引数に変数以外が含まれていると、エラーをはく。

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

## WorkFlow
### Program
#### Overview
for文でVec<Vec<Token>>(プログラム)からVec<Token>(行に当たる)を取り出す。
行ずつパースを実行していく。(どのASTNodeに当たるのかをif-elseとかで判別)
一行づつASTを返していく。

#### 詳細
- Call で呼び出すときの引数を Vec< {String, u32} > として保存する。
**Statement**
*BindVar*
- 行の最初の文字が大文字から始まる文字列(Tokenizerですでに文字列は先頭以外全て小文字)
- 次のtokenが`Identifier(":")`の時、その後の式を再パースする。
*BindFn*
- 最初のトークンが全て小文字で始まる時。
- 最初のトークンから`:`までの部分のトークンをinputとしてとる。
- 入力となるトークンの数をVec< {String, u32}>に保存する。
- `:`の後の式を再パースする。//評価の時は、この中の変数はinputのみ。
*Input*
- 最初のトークンが`Keyword`, `input`の時、その後のtokenが全て、Identifier(String)の時、全てVec<String>に入れる。
*Out*
- 最初のトークンが`Keyword`, `out`の時、その後のトークンが全て、Identifier(String)の時、全てVec<String>に入れる。
**Expression**
*Identifier*
- 最初の文字が大文字の文字列の時、その文字列をIdentifier(String)として返す。
*Literal*
- 最初の文字が`1`,`0`の時、それがLiteral(bool)として返される。
*NOT*
- 最初のトークンが`not`の時、直後の式を再パースして、とる。
*OR*
- 最初のトークンが`or`の時、直後とその後ろの２つの式を再パースする。
*Call*
- 最初のトークンが最初の文字が大文字の文字列の時、その後のトークンをリストに設定された分だけ取って返す。
- リストに保存されてなかったら、エラーリストに追加する。
*Error*
- それぞれごとにフォーマットしてエラーリストに追加する。

### Use
input: Vec<Token>
parser = Parser::new(input)
parser.parse()
ast: Vec<AST> = parser.get_ast()

## AstExamples
`
  [AST { ast_type: Bind, right: AST { ast_type: Identifier, right: }]
`
