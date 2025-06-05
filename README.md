# Clodo - Todo Manager

A simple, command-line todo list application built in Rust. Manage your tasks with commands to add, list, mark as done, edit, and delete tasks. Todos are stored in a local JSON file and displayed with colored output for better readability.

## Features

- **Add Tasks:** Create new tasks with auto-incrementing IDs.
- **List Tasks:** View all tasks with their status (Pending or Done) in color.
- **Mark as Done:** Mark tasks as completed by ID.
- **Edit Tasks:** Update the description of existing tasks by ID.
- **Delete Tasks:** Remove tasks by ID.
- **Persistent Storage:** Tasks are saved to a `todos.json` file.
- **Colored Output:** Uses the `colored` crate for visual feedback (green for success/done, red for pending/errors, yellow for warnings).

## Installation

### Option 1: Download Pre-Built Binaries

- Visit the [Releases](https://github.com/fobabs/clodo/releases) page on GitHub.
- **Linux**

  1. Download the `clodo` binary.
  2. Make it executable and move to PATH:

  ```bash
  chmod +x clodo
  sudo mv clodo /usr/local/bin/
  ```

- **Windows**

  1. Download the `clodo.exe` binary.
  2. Move it to a folder (e.g., `C:\Program Files\clodo`).
  3. Add the folder to your system PATH (via "Edit the system environment variables").

- Test it:

  ```bash
  clodo --help
  ```

### Option 2: Build From Source

1. **Prerequisites:**

- Install [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`).
- Ensure you have terminal to run commands.

2. **Setup:**

- Clone the repository:

  ```bash
  git clone https://github.com/fobabs/clodo.git
  cd clodo
  ```

- The `Cargo.toml` includes:

  ```toml
  [package]
  name = "clodo"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  colored = "2.0"
  clap = { version = "4.0", features = ["derive"] }
  ```

3. **Build:**

- Compile the project:

  ```bash
  cargo build --release
  ```

- The executable will be in `target/release/clodo` (or `clodo.exe` on Windows).

4. **Install:**

- Follow the same steps as above to move the binary to your PATH.

## Usage

Run the program with `clodo <command>`. Available commands:

- **Add a Task:**

  ```bash
  clodo add "Your task here"
  ```

  Adds a new task with an auto-assigned ID. Example output:

  ```bash
  Task added successfully!
  ```

- **List Tasks:**

  ```bash
  clodo list
  ```

  Displays all tasks. Example output:

  ```bash
  Your Todos:
  1: Write final report - ⬜ Pending
  2: Buy groceries - ✔ Done
  ```

- **Mark Task as Done:**

  ```bash
  clodo done <id>
  ```

  Marks a task as completed by ID. Example:

  ```bash
  clodo done 1
  ```

  Output: `Task marked as done!`

- **Edit a Task:**

  ```bash
  clodo edit <id> "New task description"
  ```

  Updates the description of a task by ID. Example:

  ```bash
  clodo edit 1 "Write final report draft"
  ```

  Output: `Task edited successfully!`

- **Delete a Task:**

  ```bash
  clodo delete <id>
  ```

  Removes a task by ID. Example:

  ```bash
  clodo delete 1
  ```

  Output: `Task deleted successfully!`

## Storage

- Tasks are stored in a `todos.json` file in the directory where `clodo` is run.
- Example format:

  ```json
  {
    "todos": [
      {
        "id": 1,
        "task": "Write final report",
        "done": false
      }
    ],
    "next_id": 2
  }
  ```

## Notes

- If a task ID doesn’t exist for `done`, `edit`, or `delete`, an error message is shown in red.
- The program uses the `clap` crate for command-line parsing and `colored` for output styling.
- Data persists between sessions in the `todos.json` file.

## Dependencies

- `serde` and `serde_json`: For JSON serialization and storage.
- `colored`: For colored terminal output.
- `clap`: For command-line argument parsing.

## Troubleshooting

- If `todos.json` is missing, the program starts with an empty task list.
- Ensure Rust and dependencies are installed correctly.
- Check for typos in commands or IDs.
- Ensure `clodo` is in your PATH for global use.

## Future Improvements

- Add support for TOML or YAML storage.
- Include task categories or priorities.
- Add a command to clear all tasks.
