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

## Chapter 7 — Packages, Crates, Modules

- [x] Split into `task.rs` (Task + Status), `todo_list.rs` (TodoList),
  `main.rs` (CLI loop + Command + parse_command)
- [x] Use `mod` / `use` correctly across files

## Chapter 8 — Common Collections

- [X] Switch storage from `Vec<Task>` to `HashMap<u32, Task>` keyed by an
  auto-incrementing ID
- [X] Add `list_by_status(&self, status: Status)` filtering method
- [X] Update `Command::Done`/`Remove`/`Progress` to use the new ID type

## Chapter 9 — Error Handling

- [ ] Define `enum TodoError { TaskNotFound(u32) }`
- [ ] `complete_task`, `remove_task`, `set_in_progress` return
  `Result<(), TodoError>`
- [ ] Main loop matches on `Result` and prints user-friendly errors
- [ ] Optional: richer `Command::Unknown` → distinguish "unrecognized
  command" from "missing/invalid argument"

## Chapter 10 — Generics, Traits, Lifetimes

- [ ] Implement `Display` for `Task` (clean `println!("{}", task)`)
- [ ] Optional: add `priority: u8` field, implement `PartialOrd`/`Ord`
  for sorting (note: derive order on `Status` matters if it's involved
  in comparisons)

## Chapter 11 — Automated Tests

- [ ] Refactor `list_tasks`/`list_by_status` to return `String`/`Vec<String>`
  (testable, not just printed)
- [ ] Unit tests: add/complete/remove (including not-found cases),
  status transitions, filtering

## Chapter 12 — An I/O Project

- [ ] Review CLI structure against the `minigrep` pattern (separating
  argument parsing, logic, and I/O)
- [ ] Clean up `main.rs` so it mostly just wires things together

## Chapter 13 — Iterators & Closures

- [ ] Rewrite `list_by_status` using `.filter()` with a closure
- [ ] Add `.sort_by_key()` for sorting by priority/status

## Persistence (after Ch. 12, using `serde` + `serde_json`)

- [ ] `#[derive(Serialize, Deserialize)]` on `Task` / `Status`
- [ ] `save_to_file(&self, path: &str)` and `load_from_file(path: &str)`
- [ ] Load on startup, save on `Command::Quit`

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
