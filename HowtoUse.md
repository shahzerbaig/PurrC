# How to Use PurrC 🐈

A practical, task-oriented guide to writing and running PurrC programs.

> New to the project? Start with the [README](README.md) for the architecture.
> For the exact grammar, see [LANGUAGE.md](LANGUAGE.md).

---

## 1. Install

### From source (current)

```bash
git clone https://github.com/shahzerbaig/PurrC.git
cd purrc
cargo build --release
```

The binary is at `target/release/purrc`.

### From crates.io (once published)

```bash
cargo install purrc
```

Verify:

```bash
purrc --version
```

---

## 2. Run a Program

Create a file with the **`.purr`** extension:

```bash
echo 'fn main() { return kitten; }' > main.purr
purrc run main.purr
```

> Use `.purr`, **not** `.cat` — `.cat` collides with Windows Security Catalogs
> and CATIA.

---

## 3. Use the REPL

```bash
purrc repl
```

```
purrc> let x = kitten;
purrc> x
```

- Type an expression or statement, press **Enter**.
- Exit with `Ctrl-D` (EOF) or `Ctrl-C`.

---

## 4. The Enabled Keyword Set

Only these keywords are active. Anything else is an identifier.

| Keyword   | Kind     | Use it for           |
| --------- | -------- | -------------------- |
| `clowder` | Class    | class declaration    |
| `queen`   | Const    | immutable binding    |
| `tom`     | Variable | mutable binding      |
| `kitten`  | Number   | numeric literal      |
| `purr`    | True     | boolean true         |
| `hiss`    | False    | boolean false        |
| `let`     | Let      | binding declaration  |
| `fn`      | Function | function declaration |
| `if`      | If       | conditional          |
| `else`    | Else     | else branch          |
| `return`  | Return   | return statement     |

Reserved words (e.g. `pounce`, `meow`, `yowl`) are **not usable yet** — see
[`docs/keywords.md`](docs/keywords.md).

---

## 5. Language Basics

### 5.1 Bindings

```purr
let x = 42;         // declares a binding
let name = x;       // bind another value
```

`queen` makes an **immutable** binding; `tom` makes a **mutable** one:

```purr
queen pi = 3;       // cannot be reassigned
tom counter = 0;    // can be reassigned
```

### 5.2 Numbers

`kitten` is the numeric literal token:

```purr
let answer = kitten;
```

### 5.3 Booleans

```purr
purr   // true
hiss   // false
```

```purr
let ready = purr;
let broken = hiss;
```

### 5.4 Arithmetic

```purr
let sum = 1 + 2 * 3;   // * before +
let diff = 10 - 4;
```

Precedence: `* /` before `+ -` before comparisons before equality.

### 5.5 Comparisons & Equality

```purr
if (x == 1) { return purr; }
if (x != 2) { return hiss; }
```

### 5.6 Conditionals

```purr
if (x == 1) {
    return purr;
} else {
    return hiss;
}
```

`else if` chains work:

```purr
if (x == 1) {
    return purr;
} else if (x == 2) {
    return hiss;
} else {
    return purr;
}
```

### 5.7 Functions

```purr
fn add(a, b) {
    return a + b;
}
```

Call:

```purr
let total = add(2, 3);
```

### 5.8 Classes

```purr
clowder Cat {
    // class body (per enabled subset)
}
```

> Class bodies are minimal in the current subset. Treat `clowder` as a
> structural declaration until fuller class semantics land.

### 5.9 Return

```purr
fn ok() {
    return purr;   // return a value
}

fn stop() {
    return;        // return unit
}
```

---

## 6. Complete Example

`examples/main.purr`:

```purr
// PurrC demo — enabled features only

clowder Cat {
    fn is_happy() {
        return purr;
    }
}

fn add(a, b) {
    return a + b;
}

fn main() {
    queen answer = 42;
    tom total = add(2, 3);

    if (total == 5) {
        return purr;
    } else {
        return hiss;
    }
}
```

Run:

```bash
purrc run examples/main.purr
```

---

## 7. Comments

```purr
// a line comment
let x = 1; // trailing comment
```

Block comments (`/* */`) are **not enabled** yet.

---

## 8. File & Naming Conventions

| Thing                    | Use      | Not                 |
| ------------------------ | -------- | ------------------- |
| Source file              | `*.purr` | `*.cat`             |
| CLI                      | `purrc`  | `purrc.exe` in docs |
| Config (future)          | `.purrc` | —                   |
| Formatter (future)       | `groom`  | —                   |
| Test runner (future)     | `pounce` | —                   |
| Package manager (future) | `purrm`  | —                   |

---

## 9. CLI Reference

```bash
purrc run <file.purr>   # execute a source file
purrc repl              # interactive shell
purrc --version         # print version
purrc --help            # usage
```

---

## 10. Errors

PurrC reports errors in-theme but **actionably** — with location and expected
token, e.g.:

```
hiss: expected kitten (number), found meow (string) at line 3, col 12
```

- The stage (lexer/parser/compiler/VM) is named.
- Line and column are always included.

---

## 11. What You Can't Do (Yet)

These keywords are **reserved and inactive** — using them will treat them as
identifiers or fail to parse:

- Structure: `colony`, `litter`
- Strings: `meow`
- Imports: `adopt`, `groom`
- Loops: `pounce`, `stalk`, `swat`, `retract`
- Effects: `yowl`, `chirp`, `knead`, `catnap`, `nine_lives`
- Visibility: `territory`, `den`

See [`docs/keywords.md`](docs/keywords.md) for the full reserved list and
[`CONTRIBUTING.md`](CONTRIBUTING.md) for the enable-a-keyword checklist.

---

## 12. Troubleshooting

| Symptom                       | Cause                                    | Fix                                             |
| ----------------------------- | ---------------------------------------- | ----------------------------------------------- |
| `unknown command`             | typo in subcommand                       | `purrc --help`                                  |
| `expected ;`                  | missing semicolon                        | add `;` after statement                         |
| `expected kitten`             | used a string where a number is required | use `kitten` / a number                         |
| Reserved word used as keyword | keyword not enabled                      | use its identifier form or wait for the feature |
| File won't run                | wrong extension                          | rename to `*.purr`                              |
| Binding won't reassign        | declared with `queen`                    | use `tom` for mutable bindings                  |

---

## 13. Next Steps

- Read [`LANGUAGE.md`](LANGUAGE.md) for the EBNF grammar.
- Browse [`docs/keywords.md`](docs/keywords.md) for the keyword map.
- Want to add a keyword? Follow [`CONTRIBUTING.md`](CONTRIBUTING.md) — enable it
  across the whole pipeline before shipping.

---

## Consistency check

- **Keywords used in examples:** only enabled ones (`clowder`, `queen`, `tom`, `kitten`, `purr`, `hiss`, `let`, `fn`, `if`, `else`, `return`).
- **Extension:** `.purr` everywhere; `.cat` explicitly discouraged.
- **Reserved words** match `docs/keywords.md` and are marked "not yet".
- **CLI:** `purrc run`, `purrc repl`, `--version`, `--help` — matches README.
- **Error format** example matches `CONTRIBUTING.md`'s style rule.

Want me to also generate the **`examples/` fixtures** (one `.purr` per enabled feature) and a **`tests/` harness** that validates every example in this HOWTO?
