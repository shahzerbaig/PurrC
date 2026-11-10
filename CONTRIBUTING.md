# Contributing to PurrC 🐈

Thanks for pouncing on the project. PurrC is a small, cat-themed
bytecode-compiled language written in Rust. This guide keeps contributions
consistent with the language's identity and its deliberately minimal feature set.

---

## Ground Rules

1. **Minimal surface first.** PurrC enables only the _necessary_ features. Do not
   enable a themed keyword (`pounce`, `yowl`, `meow`, …) until the lexer,
   parser, compiler, and VM all support it **end-to-end**.
2. **`token.rs` is the single source of truth.** All keywords live in the keyword
   lookup in `token.rs`. The parser, compiler, and VM must never hard-code
   keyword strings — they match on `TokenKind`.
3. **Theme with reason.** New keywords must fit the linguistic mapping:
   - _Vocalizations_ → values/effects (`purr`, `hiss`, `meow`, `yowl`, `chirp`)
   - _Behaviors_ → control flow (`pounce`, `stalk`, `swat`, `retract`, `knead`)
   - _Social/territorial_ → structure/visibility (`clowder`, `colony`, `den`, `territory`)
   - _Lifecycle_ → values/constancy (`kitten`, `queen`, `tom`, `nine_lives`)
4. **Files are `.purr`.** Never `.cat` (collides with Windows Security Catalogs
   and CATIA).
5. **Keep it small and readable.** Prefer clear, boring Rust over clever Rust.

---

## Getting Started

```bash
git clone https://github.com/shahzerbaig/PurrC.git
cd purrc
cargo build
cargo test
cargo run -- run examples/main.purr
cargo run -- repl
```
