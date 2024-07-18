# Basic Statement
There are only four statements.

## Bind
There are two types in Bind.
First is unused a argument.
Another one is used some arguments.

### No argument bind
`A : 0`
- First one is this way, can define from name, delimiter and expression.
- Name is start with upper case and others are all lower case.(ex: A, Aone)
* In the future, able delimitte with `_` and able include number like `A0`.

### Any arguments bind
`nor A B : not or A B`
- Second one is this way, can define from name, arguments(any), delimiter and expression.
- You can use only bindings with arguments defined before this binding and arguments only in the expression.

## IO
Can use `input`, `out` as IO.

### Input
`input : A B`
- Bind value that is inputed in commandline.
- You can work with multiple bindings at once.
- Cannot bind expression.

### Out
`out : A B`
- Output value of A and B to default output.
- Cannot output expression.
* Now input expression as arguments, output value of bind name from that expression. This fix in the future.
