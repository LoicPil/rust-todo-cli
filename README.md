# Rust Todo CLI

A command-line to-do list application, built while learning Rust through
"The Rust Programming Language" book (the "Rust Book"). It doubles as a
first Rust portfolio project and as a genuinely usable everyday todo tool.

## Status

Implements concepts through Chapter 15 of the Rust Book (structs, enums,
pattern matching, modules, collections, error handling, generics/traits,
automated tests, iterators/closures, JSON persistence via `serde`, and
smart pointers via `Rc<RefCell<_>>`). See `RoadMap.md` for the full
chapter-by-chapter breakdown.

## Features

- Add, list, complete, and remove tasks, with statuses `Todo` / `InProgress` / `Done`
- Filter tasks by status
- A separate **multi-list board**: named lists (e.g. "Work", "Personal")
  that can **share the same task** — completing it from one list marks it
  done everywhere it appears
- Create and delete whole lists (deleting a list unassigns its tasks
  first; a task with no list left referencing it is cleaned up entirely)
- Everything is saved as JSON and reloaded automatically between runs
- Colored, organized CLI output with a `help` menu
- Works from **any directory** — data is stored in `~/.todo_cli/`, not
  the project folder

## Usage

### Run from the project (for development)

```bash
cargo run
```

### Install it as an everyday command

To use it like a normal system command, from anywhere, without `cd`-ing
into this repo each time:

```bash
cargo install --path .
```

This builds a release binary and copies it to `~/.cargo/bin/` (already on
your `PATH` if you installed Rust via `rustup`). Afterwards, just run:

```bash
todo_list
```

from any terminal, in any folder. Your tasks live in `~/.todo_cli/todos.json`
and `~/.todo_cli/board.json`, so they persist regardless of where you launch
the program from.

To update after making changes to the code, just re-run
`cargo install --path .` — it overwrites the previously installed binary.

## Commands

**Basic list**

| Command | Description |
|---|---|
| `add <desc>` | add a new task |
| `done <id>` | mark a task done |
| `remove <id>` | remove a task |
| `progress <id>` | mark a task in progress |
| `list` | show all tasks |
| `filter <todo\|done\|inprogress>` | show tasks with a given status |

**Multi-list board**

| Command | Description |
|---|---|
| `bcreate <list>` | create an empty list |
| `badd <list> <desc>` | add a task directly into a list |
| `bdone <id>` | mark a board task done (visible in every list it's in) |
| `blists` | show every list's name |
| `blist <list>` | show tasks in a list |
| `bassign <id> <list>` | share an existing task into another list |
| `bunassign <id> <list>` | remove a task from one list only |
| `bremove <id>` | remove a task entirely, from every list |
| `bdelete <list>` | delete a whole list (unassigns its tasks first) |
| `bwhere <id>` | show which lists contain a task |

**Other**

| Command | Description |
|---|---|
| `help` / `?` | show the help menu |
| `quit` | save and exit |

## Development

```bash
cargo test        # run the test suite
cargo doc --no-deps --open   # generate and view HTML documentation
```

## Roadmap

See `RoadMap.md` for the full chapter-by-chapter plan, including what's
left: Chapter 16 (concurrency, optional), and the final multithreaded web
server project.
