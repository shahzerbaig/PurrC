# PurrC 🐈

**A small, cat-themed programming language implemented in Rust.**

PurrC is a bytecode-compiled language with a hand-written lexer, Pratt parser, tree-walking compiler, and a stack-based virtual machine. The syntax is C-family (`let`, `fn`, `if`, `else`, `return`), but the language's identity is feline — values, literals, and (in future) control-flow keywords are named after real cat vocalizations and behaviors.

> **Status:** early / minimal. Only the essential feature set is enabled. Many themed keywords are commented out in `token.rs` and reserved for later.

---

## Quick Start

**Install (once published):**

```bash
cargo install purrc
```

**Run a file:**

```bash
purrc run main.purr
```

**REPL:**

```bash
purrc repl
```

Source files use the **`.purr`** extension.

---

## Hello, World

```purr
fn main() {
    return kitten; // kitten = numeric literal
}
```

> `kitten` is the numeric literal token. `queen` is an immutable binding, `tom` is a mutable binding, `purr` is `true`, and `hiss` is `false`.

---

## The Enabled Keyword Set

Only these tokens are active right now. Everything else falls through to `TokenKind::Ident`.

| Keyword         | TokenKind  | Meaning / Rationale                       |
| --------------- | ---------- | ----------------------------------------- |
| `clowder`       | `Class`    | a group of cats = a class                 |
| `queen`         | `Const`    | a breeding female = fixed/immutable value |
| `tom`           | `Variable` | a male cat = mutable binding              |
| `kitten`        | `Number`   | a young cat = small numeric literal       |
| `purr`          | `True`     | the contentment sound = true              |
| `hiss`          | `False`    | the defensive sound = false               |
| `let`           | `Let`      | standard binding declaration              |
| `fn`            | `Function` | function declaration                      |
| `if`            | `If`       | conditional                               |
| `else`          | `Else`     | else branch                               |
| `return`        | `Return`   | return from function                      |
| _anything else_ | `Ident`    | identifier fallback                       |

### Reserved (commented out, not yet wired up)

These are sketched in the lexer but **not active**. Treat them as a roadmap, not a spec:

- **Structure:** `colony` (module), `litter` (block)
- **Values:** `meow` (string)
- **Declarations:** `adopt` (import), `groom` (use)
- **Control flow:** `pounce` (while), `stalk` (for), `swat` (break), `retract` (continue)
- **Functions/effects:** `yowl` (throw), `chirp` (yield)
- **Misc:** `knead` (await), `catnap` (async), `nine_lives` (try), `territory` (public), `den` (private)

---

## Architecture

PurrC is a classic four-stage pipeline:

```
Source (.purr)
   │
   ▼
Lexer (lexer.rs) ──► Token stream
   │
   ▼
Parser (parser.rs) ──► AST (Program)
   │
   ▼
Compiler (compiler.rs) ──► Bytecode (Instructions + constants pool)
   │
   ▼
VM (vm.rs) ──► Result / Output
```

Supporting modules:

| File          | Role                                                                                    |
| ------------- | --------------------------------------------------------------------------------------- |
| `main.rs`     | entry point; dispatches to file mode or REPL                                            |
| `file.rs`     | reads source files                                                                      |
| `repl.rs`     | interactive mode; feeds lines into the lexer                                            |
| `token.rs`    | token definitions (`TokenKind`) and keyword lookup                                      |
| `lexer.rs`    | `read_char`, `next_token`, `skip_whitespace`                                            |
| `parser.rs`   | Pratt parser; produces `Program` → `Statements` (`LetStmt`, `ExprStmt`, `InfixExpr`, …) |
| `compiler.rs` | walks the AST, emits opcodes, tracks symbols and scopes                                 |
| `bytecode.rs` | `Instructions`, `OpAdd`, `OpCall`, `OpGetGlobal`, `OpConstant`, `OpHalt`, …             |
| `vm.rs`       | stack machine; fetch → decode → execute; call frames                                    |

### Instruction cycle

```
Fetch ──► Decode ──► Execute ──► Fetch ──► … ──► Halt
```

The VM is a stack machine driven by an instruction pointer (`ip`), with a constants pool and call frames.

### Data flow

1. `file.rs` reads `.purr` source into a `String`.
2. `lexer.rs` yields `Token`s until EOF.
3. `parser.rs` builds a `Program` AST.
4. `compiler.rs` emits instructions and constants into `bytecode.rs`.
5. `vm.rs` executes the bytecode and returns a result.

---

## Project Layout

```
purrc/
├── src/
│   ├── main.rs        # entry point
│   ├── file.rs        # source loader
│   ├── repl.rs        # interactive shell
│   ├── token.rs       # TokenKind + keyword table
│   ├── lexer.rs       # tokenizer
│   ├── parser.rs      # Pratt parser + AST nodes
│   ├── compiler.rs    # AST → bytecode
│   ├── bytecode.rs    # opcodes + Instruction
│   └── vm.rs          # virtual machine
├── examples/
│   └── main.purr
└── Cargo.toml
```

---

## Design Notes for Contributors

- **Keyword table lives in `token.rs`.** To enable a reserved keyword, uncomment its match arm and add the corresponding `TokenKind` variant. Keep the theme consistent: _vocalizations → values/effects_, _behaviors → control flow_, _social/territorial → structure/visibility_.
- **File extension is `.purr`**, not `.cat`. `.cat` collides with Windows Security Catalogs and CATIA; `.purr` matches the language name.
- **Keep the enabled surface minimal.** The "few necessary features" principle: don't enable a keyword until the compiler and VM fully support it end-to-end. A reserved-but-commented keyword is better than a half-implemented one.
- **Lexer is the single source of truth** for keywords. Parser and compiler should never hard-code keyword strings.
- **Errors should stay in-theme** (e.g. `hiss: expected kitten, found meow`) but remain _actionable_ — include line/column and what was expected.

---

## Roadmap

- [ ] Enable `meow` (string literal) with proper escaping
- [ ] `pounce` / `stalk` loops + `swat` / `retract` loop control
- [ ] `yowl` (throw) and `nine_lives` (try) error handling
- [ ] `colony` modules and `adopt` imports
- [ ] `catnap` / `knead` async primitives
- [ ] Formatter (`groom`), test runner (`pounce`), package manager (`purrm`)

---

## License

TBD.
