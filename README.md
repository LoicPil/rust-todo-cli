# Todo List (Rust learning project)

A simple command-line to-do list application, built while learning Rust 
through "The Rust Programming Language" book (the "Rust Book").

## Status

This is my first Rust project, currently implemented using concepts from 
Chapters 1–5 (variables, ownership, structs, methods, basic collections, 
and basic I/O). It will be progressively extended as I work through the 
rest of the book — see commit history for the evolution.

## Features (current)

- Add tasks
- List tasks with completion status
- Mark tasks as done
- Remove tasks
- Simple interactive command-line interface

## Usage

```bash
cargo run
```

Available commands:
- `add <description>` — add a new task
- `list` — show all tasks
- `done <id>` — mark a task as done
- `remove <id>` — remove a task
- `quit` — exit the program

## Roadmap

- [ ] Replace boolean status with an enum (Todo / In Progress / Done)
- [ ] Split code into modules
- [ ] Custom error handling for invalid commands/indices
- [ ] Persistent storage (save/load to a file)
- [ ] Unit tests
