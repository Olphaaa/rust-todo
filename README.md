# Todo Rust CLI

A simple command-line todo application written in Rust.

## Features

- Add tasks with optional descriptions
- List tasks with status icons
- Remove tasks by ID
- Mark tasks as done or undone
- Show details of a task
- Print all tasks as JSON
- Persistent storage in `tasks.json`
- Short command aliases for faster usage

## Usage

Build the project:

```sh
cargo build --release
```

Run commands:

```sh
# Add a task
todo add "Buy groceries" "Milk, eggs, bread"
todo a "Read book"

# List tasks
todo list
todo l

# Remove a task
todo remove 1
todo r 1

# Mark as done
todo done 2
todo do 2

# Mark as undone
todo undo 2
todo u 2

# Show details
todo describe 2
todo d 2

# Print all tasks as JSON
todo json
todo j
```

## Data

Tasks are stored in `tasks.json` in the current directory.

## Requirements

- Rust (edition 2024)
- [clap](https://crates.io/crates/clap)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)
