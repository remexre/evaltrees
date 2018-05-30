# Syntax Accepted By Evaltrees

Evaltrees accepts a series of declarations, as well as an expression to evaluate.
Their syntax is described as follows:

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
