# Changelog

All notable changes to this project are documented here. The format is
based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and
version numbers follow [Semantic Versioning](https://semver.org/):
`MAJOR.MINOR.PATCH`.

Since this project is built chapter-by-chapter through *The Rust
Programming Language* book, minor version bumps generally correspond to
finishing a chapter's worth of features.

## [Unreleased]

- Chapter 16 (optional): background thread auto-saving to disk every N
  seconds, using `std::thread` + `Arc<Mutex<_>>`.

## [0.15.0] - 2026-07-23

### Added
- Multi-list `Board`: named lists (e.g. "Work", "Personal") that can
  **share the same task** via `Rc<RefCell<Task>>` — completing a task
  from one list marks it done in every list it belongs to (Chapter 15).
- Board commands: `bcreate`, `badd`, `bdone`, `blists`, `blist`,
  `bassign`, `bunassign`, `bremove`, `bdelete`, `bwhere`.
- `TodoError::ListNotFound`, plus a `Display` impl for `TodoError` so
  error messages are printed uniformly across the app.
- Board persistence to `board.json`, via a plain serializable shadow
  struct (`Rc<RefCell<Task>>` can't derive `Serialize`/`Deserialize`
  directly; the shadow struct sidesteps that while still preserving
  which lists share which tasks).
- App data now lives in `~/.todo_cli/` instead of the current working
  directory, so the CLI behaves the same regardless of where it's run
  from — a prerequisite for installing it as an everyday command via
  `cargo install --path .`.
- Colored, sectioned CLI output (`colored` crate) and a `help` / `?`
  command replacing the old always-printed one-line command list.
- Rustdoc (`///`) comments across `task.rs`, `todo_list.rs`, `board.rs`,
  and `main.rs`; browsable via `cargo doc --no-deps --open`.

## [0.13.0] - Chapters 13 & Persistence

### Added
- `list_tasks` / `list_by_status` now sort by ascending task id before
  formatting, fixing `HashMap`'s unordered iteration (Chapter 13).
- JSON persistence for the single-list `TodoList`: `save_to_file` /
  `load_from_file`, loading on startup (falling back to an empty list if
  no save file exists yet) and saving on `quit`, via `serde` +
  `serde_json`.

## [0.11.0] - Chapters 8-11

### Added
- Storage switched from `Vec<Task>` to `HashMap<u32, Task>` keyed by an
  auto-incrementing, never-reused id (Chapter 8).
- `list_by_status` filtering method.
- `TodoError::TaskNotFound`; `complete_task` / `remove_task` /
  `set_in_progress` now return `Result<(), TodoError>` instead of
  printing directly (Chapter 9).
- `Command::Unrecognized` / `Command::InvalidArgument` replacing a
  single catch-all `Unknown` variant, distinguishing an unknown command
  name from a known command with a bad argument.
- `Display` impl for `Status` (icons `[]` / `[~]` / `[x]`) and `Task`
  (Chapter 10).
- Unit test suite covering add, complete, remove, set-in-progress, and
  filtering, including not-found error cases (Chapter 11).

### Notes
- Chapter 12 (An I/O Project / `minigrep` pattern) evaluated and marked
  not applicable: this is a REPL-style interactive program rather than a
  one-shot CLI tool, so there's no argument-parsing/logic/I/O split to
  extract in the same way.

## [0.7.0] - Chapters 6-7

### Added
- `Status` enum (`Todo` / `InProgress` / `Done`) replacing a plain
  `done: bool` field (Chapter 6).
- `Command` enum with payloads, and a `parse_command` function driving a
  single `match` in the main loop, replacing ad-hoc string parsing.
- Project split into modules: `task.rs`, `todo_list.rs`, `main.rs`
  (Chapter 7).

## [0.1.0] - Chapters 1-5

### Added
- Initial version: `Task` struct, `TodoList` wrapping a `Vec<Task>`.
- `add_task`, `remove_task`, `complete_task`, `list_tasks`.
- Interactive CLI loop reading from `std::io::stdin`.

[Unreleased]: https://github.com/LoicPil/rust-todo-cli/compare/v0.15.0...HEAD
[0.15.0]: https://github.com/LoicPil/rust-todo-cli/releases/tag/v0.15.0
