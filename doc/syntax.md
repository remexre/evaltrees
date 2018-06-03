# Syntax Accepted By Evaltrees

Evaltrees accepts a series of declarations, as well as an expression to evaluate.
Their syntax is described informally as follows (All operator precedences and associativities follow the OCaml rules):

```plain
decl ::= name pattern* '=' expr

pattern ::= name
         |  literal
         |  pattern '::' pattern
         |  '(' pattern ')'

expr ::= name
      |  literal
      |  expr expr+
      |  expr binop expr
      |  '(' expr ')'
      |  '[' expr (',' expr)* ']'

binop ::= '+' | '-' | '*' | '/' | 'mod' | '::'

literal ::= '[]' | digit+
```

The actual grammar used is [here](../../src/cst/parser/grammar.lalrpop).
