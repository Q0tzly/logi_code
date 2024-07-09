# The LogiCode Programming Language - Pre 0.5.0
LogiCodeは、組み合わせ回路を定義、実行するためのスクリプト言語です。シンプルなシンタックスと最小限の演算子を提供します。

## 特徴
- シンプルなシンタックス
- 最小限の演算子
- bool型のみのサポート
- ポーランド記法による、組み合わせ回路の定義

## インストール
LogiCodeをコマンドラインツールとしてインストールするには、以下のコマンドを実行します。
Cargoがインストールされているのが前提です。
``` sh
git clone https://github.com/Q0tzly/logi_code.git
cd logi_code
cargo install --path .
```

## 実行方法
拡張子は`.lc`です。
``` sh
logi run <path_to_your_file.lc>
```

## シンタックス
LogiCodeのシンタックスは非常にシンプルです。文は四つしかありません。
以下に基本的な構成要素を説明します。

### 文
#### Bind
Bindには二種類あります。一つは引数を伴わないバインド。もう一つは引数を伴うバインド。

**引数を伴わないバインド**
`A : 0`
- 一つ目のバインドはこのように名前、区切り文字、式のように定義できます。
- 名前は最初の文字が大文字で始まって、後ろは全て小文字となります。(ex: A, Aone)
* 将来的には`_`で区切ったり、`A0`のように数字も含めることができるようにするつもりです。

**引数を伴わないバインド**
`nor A B : not or A B`
- 二つ目のバインドはこのように名前、引数(複数可)、区切り文字、式のように定義できます。
- 式に使えるのは、このバインドより前に定義されていた引数を伴うバインドと、引数のみです。

#### IO
IOとして、`input`, `out`を使用することができます。

**input**
`input : A B`
- コマンドラインで入力した値をバインドします。
- 複数のバインドを一度に扱うことができます。
- 式はバインドすることができません。

**out**
`out : A B`
- A, Bの値を標準出力に出力します。
- 式は出力することができません。
* 現在は式が入力されるとその中に含まれているバインド名の値が出力されますが、今後修正します。

### 式
式は全てポーランド記法で記述します。括弧は使用しません。

#### 演算子
演算子は二つのみサポートしています。

**or**
`or A B`
- 引数は二つ取ります。
- AとBの論理和を返します。

**not**
`not A`
- 引数は一つ取ります。
- Aの否定を返します。

#### リテラル
`A : 0`
- bool型のリテラルはそれぞれ`0`, `1`として扱います。

## 例
詳細な例は[examples](https://github.com/Q0tzly/logi_code/tree/main/examples)ディレクトリにあります。

## 貢献
貢献を希望する場合は、プルリクエストを送信してください。

## ライセンス
LogiCodeはApache Licenseです。
