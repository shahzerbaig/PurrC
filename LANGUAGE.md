## PurrC Language Reference (Enabled Subset)

---

This document specifies **only the currently enabled features** of PurrC.
Reserved-but-inactive keywords are listed in `docs/keywords.md` and are
**not** part of the grammar below.

---

## Lexical Grammar

Grammar reference (enabled subset, EBNF)

### Source files

- Extension: `.purr`
- Encoding: UTF-8
- Line comments: `//` to end of line
- (Block comments `/* */` are not enabled yet.)

### Whitespace & comments

```
whitespace = " " | "\t" | "\r" | "\n" ;
comment    = "//" , { any char except newline } ;
```

### Identifiers

```
ident = ( letter | "_" ) , { letter | digit | "_" } ;
letter = "A".."Z" | "a".."z" ;
digit  = "0".."9" ;
```

An identifier that matches a keyword is lexed as that keyword.

### Numeric literals

```
number = digit , { digit } ;
```

(`kitten` is the token name for a numeric literal.)

### Operators & punctuation (enabled)

```
=  +  -  *  /  (  )  {  }  ,  ;
```

---

## Keywords (enabled)

| Keyword   | TokenKind  | Role                 |
| --------- | ---------- | -------------------- |
| `clowder` | `Class`    | class declaration    |
| `queen`   | `Const`    | immutable binding    |
| `tom`     | `Variable` | mutable binding      |
| `kitten`  | `Number`   | numeric literal      |
| `purr`    | `True`     | boolean true         |
| `hiss`    | `False`    | boolean false        |
| `let`     | `Let`      | binding declaration  |
| `fn`      | `Function` | function declaration |
| `if`      | `If`       | conditional          |
| `else`    | `Else`     | else branch          |
| `return`  | `Return`   | return statement     |

---

## Syntactic Grammar (EBNF)

```
Program        = { Statement } EOF ;

Statement      = LetStmt
               | FnDecl
               | IfStmt
               | ReturnStmt
               | ExprStmt ;

LetStmt        = "let" , ident , "=" , Expr , ";" ;
FnDecl         = "fn" , ident , "(" , [ ident , { "," , ident } ] , ")" ,
                 Block ;
IfStmt         = "if" , "(" , Expr , ")" , Block ,
                 [ "else" , ( IfStmt | Block ) ] ;
ReturnStmt     = "return" , [ Expr ] , ";" ;
ExprStmt       = Expr , ";" ;

Block          = "{" , { Statement } , "}" ;

Expr           = Equality ;
Equality       = Comparison , { ( "==" | "!=" ) , Comparison } ;
Comparison     = Term , { ( "<" | ">" | "<=" | ">=" ) , Term } ;
Term           = Factor , { ( "+" | "-" ) , Factor } ;
Factor         = Unary , { ( "*" | "/" ) , Unary } ;
Unary          = [ "-" | "!" ] , Primary ;
Primary        = "kitten"
               | "purr"
               | "hiss"
               | ident
               | "(" , Expr , ")"
               | Call ;
Call           = ident , "(" , [ Expr , { "," , Expr } ] , ")" ;
```

> Precedence is implemented via Pratt parsing in `parser.rs`. The levels above
> reflect the intended precedence: `* /` > `+ -` > comparisons > equality.

---

## Semantics

### Bindings

- `let x = expr;` declares a binding (token `Let`).
- `queen` declares an **immutable** binding; reassignment is a compile error.
- `tom` declares a **mutable** binding.

### Booleans

- `purr` → true
- `hiss` → false

### Numbers

- `kitten` → integer literal.

### Functions

- `fn name(params) { ... }`
- `return expr;` returns a value; bare `return;` returns unit.

### Conditionals

- `if (cond) { ... } else { ... }`; `else if` chains are allowed.

---

## Comments

```purr
// a line comment
let x = 1; // trailing comment
```

---
