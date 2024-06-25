# Logi Code - Overview
Logi Codeは、組み合わせ回路を定義、実行できるスクリプト言語。
シンプルでわかりやすいシンタックスと最低限の演算子を提供して、組み合わせ回路の学習ツールとして利用できる。

## Purpose
- 組み合わせ回路の学習ツールとして使用できる。
- わかりやすくシンプルなシンタックスを提供する。
- シンプルな処理系を実現する。

## Features
### Summary
- 1bitの真偽値のみをサポート
- 式の定義においてポーランド記法を採用
- 必要最低限の演算子を提供
- 式指向のシンタックス

### Syntax
- 全ての式は一行で書かなければいけない。

#### Bind
**Variable**
`A : B`
- `A`に`B`をバインドする。ここで`B`は式や値を表す。
- 入力を伴わない式や値をバインドする時のバインド名は最初の文字だけ大文字で他は小文字にする。
**Function**
`and A B : not or not A not B`
- `and`に入力`A`, `B`を伴う式をバインドする。
- ここでの`A`や`B`は定義時に値が代入されず、コール時にinputとしておいたバインドの値が代入される。
- 入力を伴うバインド名は全て小文字にする。

#### Operator
- `not A` : Aの否定を返す。
- `or A B` : AとBの論理和を返す。

#### IO
`A : input` : 標準入力の値をAにバインドする。
`out A` : Aの値を標準出力に表示する。
- `input`も`out`もバインド名にしか作用しない。(演算子を含んだ式を置いてはいけない。)

#### Comment
`#で始まって行末までがコメント`

### Usage
`logi` : 対話型インタプリタが起動
`logi [file_name.lc]` : インタプリタによってファイルに書かれたコードが実行される。
`logi build [file_name]` :ASTが`file_name.lca`として出力される。
`logi run [file_name.lca]` : `file_name.lca`が実行される。

### Examples
`
and A B : not or not A not B
A : input
B : input
C : and A B
out C
`
