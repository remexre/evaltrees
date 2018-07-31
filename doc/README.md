# Documentation

## API

See [docs.rs](https://docs.rs/evaltrees).

## Syntax

Evaltrees accepts a series of declarations, as well as an expression to evaluate.
Their syntax is described informally as follows (All operator precedences and associativities follow the OCaml rules):

```plain
decls ::= (decl ";;")*

decl ::= name pattern2* "=" expr

pattern ::= pattern2 "::" pattern
         |  pattern2
pattern2 ::= name
          |  literal
          |  "(" pattern ")"

expr ::= name
      |  literal
      |  expr expr+
      |  expr binop expr
	  |  "if" expr "then" expr "else" expr
      |  "(" expr ")"
      |  "[" expr ("," expr)* "]"

binop ::= "+" | "-" | "*" | "/" | "mod" | "::"

literal ::= "[]" | digit+ | "true" | "false"
```

The actual grammar used is [here](../src/cst/parser/grammar.lalrpop).
