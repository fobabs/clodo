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

1. **Prerequisites:**

- Rust 1.56 or later is required: [Install Rust](https://www.rust-lang.org/tools/install).
- A terminal that supports ANSI color codes for the best experience.

2. **Install clodo using Rust's package manager, cargo:**

  ```bash
  cargo install clodo
  ```

3. **Test it:**

  ```bash
  clodo --help
  ```

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

## Troubleshooting

- If `todos.json` is missing, the program starts with an empty task list.
- Ensure Rust is installed.
- Check for typos in commands or IDs.

## Future Improvements

- Add support for TOML or YAML storage.
- Include task categories or priorities.
- Add a command to clear all tasks.

## License

`clodo` is licensed under either the MIT License or the Apache License 2.0, at your option. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request. See [CONTRIBUTING.md](CONTRIBUTING.md) file for details.
