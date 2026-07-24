<div align="center">

# 🦀 Rust Todo CLI

**A colorful, multi-list command-line todo app — built from scratch while learning Rust.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-1.0.0-blue)](CHANGELOG.md)
[![Built from the Rust Book](https://img.shields.io/badge/built%20from-The%20Rust%20Book-orange)](https://doc.rust-lang.org/book/)

</div>

---

## What is this?

A command-line todo list, built chapter-by-chapter while working through
[*The Rust Programming Language*](https://doc.rust-lang.org/book/) —
and by the end, a genuinely useful everyday tool, not just a tutorial
exercise. It went from a 5-minute `Vec<Task>` prototype to a
persisted, multi-list, color-coded CLI with its own installable binary.

```
╭──────────────────────────────────╮
│      🦀 RUST TODO CLI 🦀         │
╰──────────────────────────────────╯
Type help for the list of commands.

╭─ All tasks (3)
╭─────┬───────────┬──────────────────╮
│ ID  │ Status    │ Description      │
├─────┼───────────┼──────────────────┤
│ 1   │ [x] Done  │ Set up project   │
│ 2   │ [~] Doing │ Write the tests  │
│ 3   │ [ ] Todo  │ Ship it          │
╰─────┴───────────┴──────────────────╯
```

## ✨ Features

- 📝 **Basic todo list** — add, complete, remove, mark in-progress, filter by status
- 🗂️ **Multi-list board** — organize tasks into named lists (e.g. "Work", "Personal"), with tasks that can be **shared across lists** (complete it in one place, it's done everywhere)
- 💾 **Persistent** — everything saves to JSON automatically, reloads on startup
- 🎨 **Styled CLI** — bordered tables, color-coded status, strikethrough for done tasks
- 🌍 **Works from anywhere** — install it once, run `todo_list` from any folder on your system
- ✅ **Tested** — unit tests covering core logic and edge cases
- 📚 **Documented** — full rustdoc comments, browsable as HTML

## 🚀 Quick start

Install it as an everyday command:

```bash
git clone https://github.com/LoicPil/rust-todo-cli.git
cd rust-todo-cli
cargo install --path .
```

Then run it from anywhere:

```bash
todo_list
```

Your data lives in `~/.todo_cli/`, independent of wherever you launch
the program from.

### Or just run it locally (for development)

```bash
cargo run
```

## 📖 Commands

<details>
<summary><strong>Basic list</strong></summary>

| Command | Description |
|---|---|
| `add <desc>` | add a new task |
| `done <id>` | mark a task done |
| `remove <id>` | remove a task |
| `progress <id>` | mark a task in progress |
| `list` | show all tasks |
| `filter <todo\|done\|inprogress>` | show tasks with a given status |

</details>

<details>
<summary><strong>Multi-list board</strong></summary>

| Command | Description |
|---|---|
| `bcreate <list>` | create an empty list |
| `badd <list> <desc>` | add a task directly into a list |
| `bdone <id>` | mark a board task done (visible in every list it's in) |
| `bprogress <id>` | mark a board task in progress |
| `blists` | show every list's name |
| `blist <list>` | show tasks in a list |
| `ball` | show every list, each with its own header, in one go |
| `bassign <id> <list>` | share an existing task into another list |
| `bunassign <id> <list>` | remove a task from one list only |
| `bremove <id>` | remove a task entirely, from every list |
| `bdelete <list>` | delete a whole list (unassigns its tasks first) |
| `bwhere <id>` | show which lists contain a task |

</details>

<details>
<summary><strong>Other</strong></summary>

| Command | Description |
|---|---|
| `help` / `?` | show the help menu |
| `quit` | save and exit |

</details>

## 🧠 What this project demonstrates

Built as a deliberate tour through Rust's core concepts, chapter by
chapter:

| Concept | Where |
|---|---|
| Structs & enums | `Task`, `Status` |
| Pattern matching | `Command` parsing, the main REPL loop |
| Modules | `task` / `todo_list` / `board` / `main` |
| Collections | `HashMap<u32, Task>`, `HashSet<u32>` |
| Error handling | `TodoError`, `Result<(), TodoError>` throughout |
| Generics & traits | `Display` impls, `PartialEq`, `Clone` |
| Automated tests | unit tests across every module |
| Iterators & closures | `.filter()`, `.sort_by_key()`, `.map()` |
| Smart pointers | `Rc<RefCell<Task>>` for cross-list task sharing |
| Serialization | `serde` + `serde_json` for JSON persistence |

## 🛠️ Development

```bash
cargo test                    # run the test suite
cargo doc --no-deps --open    # generate & view HTML docs
cargo install --path . --force  # rebuild and reinstall after changes
```

## 🗺️ Roadmap & history

See [`RoadMap.md`](RoadMap.md) for the chapter-by-chapter build plan,
and [`CHANGELOG.md`](CHANGELOG.md) for a full history of what shipped
when.

The core project is considered **stable at `v1.0.0`**. Chapter 16
(concurrency — a background auto-save thread) remains an optional
future addition.

## 📄 License

[MIT](LICENSE) — do what you want with it.

---

<div align="center">

Built by [Loïc Pilette](https://github.com/LoicPil) while learning Rust, one chapter at a time.

</div>
