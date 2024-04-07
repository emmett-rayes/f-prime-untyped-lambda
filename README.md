## F-Prime

A simple implementation of multiple System F-based languages.

An implementation consists of a parser, an evaluator and, when applicable, a type checker which you can interact with using the REPL

```
cargo run -p repl
```

### Languages
The following languages are currently implemented

#### The Untyped Lambda Calculus
- Internally modeled using De Bruijn indices
- Pretty printers for named variables, De Bruijn indices, and nameless locals
- Call by value and full beta reduction evaluators
