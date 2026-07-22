# Todo List — Project Roadmap

This document tracks how the project evolves chapter by chapter through
"The Rust Programming Language" book, and what the final result will look
like.

## Chapters 1–5 (done)

- `Task` struct, `TodoList` struct wrapping `Vec<Task>`
- Methods: `add_task`, `remove_task`, `complete_task`, `list_tasks`
- Interactive CLI loop using `std::io::stdin`, `splitn`, `parse::<usize>()`

---

## Chapter 6 — Enums & Pattern Matching (done)

- [x] `Status` enum (`Todo`, `InProgress`, `Done`, `#[derive(Clone, Copy)]`),
  replacing `done: bool`
- [x] `set_in_progress(index)` to move a task to `InProgress`
- [x] `Command` enum with payloads (`Add(String)`, `Done(usize)`,
  `Remove(usize)`, `Progress(usize)`, `List`, `Quit`, `Unknown`)
- [x] `parse_command(input: &str) -> Command` for input parsing
- [x] Main loop is a single `match` on `Command`
- [x] Bonus: `remove_task` now bounds-checks instead of panicking
  (pre-empts part of Ch. 9)
- [x] Bonus: `list_tasks` handles the empty-list case

## Chapter 7 — Packages, Crates, Modules (done)

- [x] Split into `task.rs` (Task + Status), `todo_list.rs` (TodoList),
  `main.rs` (CLI loop + Command + parse_command)
- [x] Use `mod` / `use` correctly across files

## Chapter 8 — Common Collections (done)

- [x] Switch storage from `Vec<Task>` to `HashMap<u32, Task>` keyed by an
  auto-incrementing ID (`next_id` counter — IDs stay stable across deletions)
- [x] Add `list_by_status(&self, status: Status)` filtering method
- [x] Update `Command::Done`/`Remove`/`Progress` to use the new ID type (`u32`)

## Chapter 9 — Error Handling (done)

- [x] Define `enum TodoError { TaskNotFound(u32) }` (`#[derive(PartialEq, Debug)]`)
- [x] `complete_task`, `remove_task`, `set_in_progress` return
  `Result<(), TodoError>`
- [x] Main loop matches on `Result` and prints user-friendly errors
- [x] Richer command parsing → `Command::Unrecognized(String)` (unknown
  command name) split from `Command::InvalidArgument(String)` (bad/missing
  argument), instead of a single `Unknown` variant

## Chapter 10 — Generics, Traits, Lifetimes (done)

- [x] Implement `Display` for `Status` (icons: `[]`, `[~]`, `[x]`) and `Task`
  (`"{status} {description}"`)
- [ ] Optional: `priority: u8` field + `PartialOrd`/`Ord` — not done, skipped
  as optional

## Chapter 11 — Automated Tests (done)

- [x] Refactor `list_tasks`/`list_by_status` to return `Vec<String>`
  (testable, not just printed) via a shared `print_lines` helper
- [x] Unit tests: add, complete (+ not-found case), remove (+ not-found case),
  set_in_progress, filter by status

## Chapter 12 — An I/O Project (not applicable)

- Evaluated and deemed not applicable: this is a REPL-style interactive
  program rather than a one-shot CLI tool like `minigrep`, so there's no
  argument-parsing/logic/I/O split to extract in the same way. Skipped by
  design, not an oversight.

## Chapter 13 — Iterators & Closures (done)

- [x] `list_by_status` already used `.filter()` with a closure (done as part
  of Ch. 8/11 work)
- [x] `list_tasks` and `list_by_status` now collect into a `Vec<(&u32, &Task)>`
  and `.sort_by_key(|(id, _)| **id)` before formatting, fixing the
  previously unordered `HashMap` iteration

## Persistence (using `serde` + `serde_json`) (done)

- [x] `#[derive(Serialize, Deserialize)]` on `Task`, `Status`, and `TodoList`
- [x] `save_to_file(&self, path: &str) -> io::Result<()>` and
  `load_from_file(path: &str) -> io::Result<TodoList>`
- [x] Load on startup (`TodoList::load_from_file`, falling back to
  `TodoList::new()` if the file doesn't exist yet), save on `Command::Quit`
- [x] Round-trip test (save → load → assert) and missing-file test
- Known rough edge, deliberately left as-is for now: `serde_json::Error` is
  converted to `io::Error` via `.map_err(..., io::ErrorKind::Other)` inside
  `save_to_file`/`load_from_file` since `?` can't chain the two error types
  directly. A cleaner fix (a dedicated `AppError` enum with `From` impls) is
  a good candidate mini-exercise once comfortable with Ch. 9 patterns again.

## Chapter 15 — Smart Pointers (optional)

- [ ] Multiple named lists (e.g. "Work"/"Personal") sharing tasks via
  `Rc<RefCell<Task>>`

## Chapter 16 — Fearless Concurrency (optional)

- [ ] Background thread auto-saving the list to disk every N seconds
  (`std::thread` + `Mutex`)

---

## Final project recap

By the end of this roadmap, the project will be a **command-line to-do
application** with:

- **Rich task model**: each task has a description and a status
  (`Todo` / `InProgress` / `Done`), optionally a priority
- **Clean architecture**: code split across modules (`task`, `todo_list`,
  `main`), with a dedicated `Command` enum separating input parsing from
  business logic
- **Efficient storage**: tasks stored in a `HashMap<u32, Task>` for
  O(1) lookup/removal by ID
- **Robust error handling**: no panics on invalid input — errors are
  represented with a custom `TodoError` type and handled via `Result`
- **Idiomatic formatting & iteration**: tasks implement `Display`;
  filtering and sorting use iterator chains with closures
- **Persistence**: tasks are saved to a JSON file and reloaded on startup
- **Test suite**: `cargo test` verifies core behavior, including edge cases
- *(optional)* multiple lists and background auto-save for extra practice
  with smart pointers and concurrency

The end result is a small but genuinely functional, well-structured Rust
CLI application — a solid first portfolio project demonstrating ownership,
error handling, modularity, and testing.
