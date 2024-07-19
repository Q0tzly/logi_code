# AND Sample
Define circuit of and with two arguments in this chapter.

## Make file
make `and.lc` file in any directory.
Edit this file.

## Define AND Logic
To define AND logic with two arguments,
you can define it by negating input 1 and input 2, adding them together, and then negating that.
That is, in Logi Code it is represented. (arguments is A and B.)

``` logi
and A B : not or not A not B
```
It is written like this in Polish notation.

### Naming rules
Bind include arguments that expression all with lowercase.
Bind like arguments and variable that is first letter is capitalized and all subsequent letters are lowercase.

## Bind of Input
Complete defining and logic, next one is using this circuit.
Arguments to AND logic is inputting by user.

``` logi
input : A B
```
This, bind to A and B from input result by user.

## Using AND Logic
Input to AND circuit. Bind output to C.
By the way, A and B here are different from A and B when you define and.
So able any name.

``` logi
C : and A B
```

## Output
Able bind to C of AND of A and B, next output this one.

``` logi
out : C
```
Then, output value of C.

## Summary
Summary codes to this point
Use comment to able know what curcuit

``` logi
# AND Logic

// Define AND curcuit
and A B : not or not A not B

// Define inputs
input : A B

// Define result
C : and A B

// Output result
out : C
```

## Run
Run next worte code.
Run this command to run.

``` sh
logi run and.lc
```
Runed, Output sign like under one.
`> A □ B □`<br>
Switch true or false by space key, switch to next bind by tab key.
Decide by enter, end of inputing.

Result outputed.
