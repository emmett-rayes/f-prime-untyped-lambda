# F-Prime Untyped Lambda

An implementation of the untyped lambda calculus including a parser, an evaluator, and a REPL.

> Originally meant to implement multiple languages building up to System F and beyond.
> After the untyped lambda calculus, I decided to move to a Scala implementation.
> See [emmett-rayes/F-Prime](https://github.com/emmett-rayes/F-Prime).


## Features

- Terms modeled internally with De Bruijn indices
- Pretty printers for named variables, De Bruijn indices, and nameless locals
- Call-by-value and full beta-reduction evaluators


## Usage

```
cargo run -p repl
```

Enter lambda terms at the `>>` prompt (`\`, `λ`, and `@` all work as lambda).
Each reduction step is printed in order. Here, the successor function is applied
to the Church numeral zero, reducing to one:

```
>> (\n. \s. \z. s (n s z)) (\s. \z. z)
0. (λn. λs. λz. s (n s z)) (λs. λz. z)
1. λs. λz. s ((λs. λz. z) s z)
2. λs. λz. s ((λz. z) z)
3. λs. λz. s z
```

`stuck!` means the term is already in normal form and cannot be reduced further:

```
>> x (\y. y)
stuck!
0. x (λy. y)
```
