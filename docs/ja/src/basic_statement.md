# 文
文は四つしかありません。

## バインド
バインドには二種類あります。
一つは引数を伴わないバインド。
もう一つは引数を伴うバインド。

### 引数を伴わないバインド
`A : 0`
- 一つ目のバインドはこのように、名前、区切り文字、式のように定義できます。
- 名前は最初の文字が大文字で始まり、後ろは全て小文字となります。(ex: A, Aone)
* 将来的には、`_`で区切ったり、`A0`のように数字を含めることができるようにする予定です。

### 引数を伴うバインド
`nor A B : not or A B`
- 二つ目のバインドは、このように名前、引数(複数可)、区切り文字、式のように定義できます。
- 式に使えるのは、このバインドより前に定義されていた引数を伴うバインドと、このバインドに含まれる引数のみです。

## IO
IOとして、`input`、`out`を使用することができます。

### Input
`input : A B`
- コマンドラインで入力した値をバインドします。
- 複数のバインドを一度に扱うことができます。
- 式はバインドすることができません。

### Out
`out : A B`
- A、Bの値を標準出力に出力します。
- 式は出力することができません。
* 現在は式が入力されるとその中に含まれるバインド名の値が出力されますが、今後修正予定です。
