# PurrC Keyword Reference

Status legend:

- ✅ **Enabled** — lexed, parsed, compiled, executed.
- 🟡 **Reserved** — sketched in `token.rs` but commented out; not active.
- ⛔ **Rejected** — considered and dropped.

Source of truth: the keyword lookup in `src/token.rs`.

---

## Enabled Keywords

| Keyword   | TokenKind  | Theme        | Meaning                           |
| --------- | ---------- | ------------ | --------------------------------- |
| `clowder` | `Class`    | social       | a group of cats = a class         |
| `queen`   | `Const`    | lifecycle    | breeding female = fixed/immutable |
| `tom`     | `Variable` | lifecycle    | male cat = mutable binding        |
| `kitten`  | `Number`   | lifecycle    | young cat = small numeric literal |
| `purr`    | `True`     | vocalization | contentment sound = true          |
| `hiss`    | `False`    | vocalization | defensive sound = false           |
| `let`     | `Let`      | core         | binding declaration               |
| `fn`      | `Function` | core         | function declaration              |
| `if`      | `If`       | core         | conditional                       |
| `else`    | `Else`     | core         | else branch                       |
| `return`  | `Return`   | core         | return statement                  |

---

## Reserved Keywords (commented out in `token.rs`)

| Keyword      | Intended TokenKind | Theme        | Rationale                             |
| ------------ | ------------------ | ------------ | ------------------------------------- |
| `colony`     | `Module`           | social       | feral cat colony = module/namespace   |
| `litter`     | `Block`            | social       | a litter = block/scope                |
| `meow`       | `String`           | vocalization | general vocalization = string literal |
| `adopt`      | `Import`           | social       | adopt a cat = import                  |
| `groom`      | `Use`              | behavior     | grooming = bringing in / use          |
| `pounce`     | `While`            | behavior     | repeated pouncing = loop              |
| `stalk`      | `For`              | behavior     | stalking = iterate                    |
| `swat`       | `Break`            | behavior     | knock out of loop = break             |
| `retract`    | `Continue`         | behavior     | retract claws = continue              |
| `yowl`       | `Throw`            | vocalization | loud distress call = throw            |
| `chirp`      | `Yield`            | vocalization | trill = yield a value                 |
| `knead`      | `Await`            | behavior     | rhythmic push = await                 |
| `catnap`     | `Async`            | behavior     | short sleep while alert = async       |
| `nine_lives` | `Try`              | lifecycle    | nine lives = fault tolerance          |
| `territory`  | `Public`           | territorial  | marked territory = public             |
| `den`        | `Private`          | territorial  | hidden den = private                  |

> Reserved keywords must **not** be relied on in programs. They are not lexed as
> keywords until enabled across the full pipeline (see `CONTRIBUTING.md`).

---

## Rejected / Discarded

| Candidate        | Why rejected                                                   |
| ---------------- | -------------------------------------------------------------- |
| `.cat` extension | collides with Windows Security Catalogs and CATIA; use `.purr` |
| `MonkeyC` name   | collides with Garmin's Monkey C language                       |
| `CatC` name      | generic; less distinctive than `PurrC`                         |

---

## Naming Map (language → tooling)

| Concept                  | Name         |
| ------------------------ | ------------ |
| Language                 | PurrC        |
| CLI                      | `purrc`      |
| Source file              | `*.purr`     |
| Config                   | `.purrc`     |
| Formatter (future)       | `groom`      |
| Test runner (future)     | `pounce`     |
| Package manager (future) | `purrm`      |
| REPL                     | `purrc repl` |
