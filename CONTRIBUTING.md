# Contributing to clodo

Thank you for your interest in contributing to clodo, a simple command-line todo list manager written in Rust! We welcome contributions from the community to improve the project. This document outlines the process for contributing, setting up your development environment, and submitting changes.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contribution Guidelines](#contribution-guidelines)
- [Submitting Changes](#submitting-changes)
- [Licensing](#licensing)
- [Code of Conduct](#code-of-conduct)
- [Contact](#contact)

## Getting Started

`clodo` is a Rust-based CLI tool for managing todo lists with commands like `add`, `list`, `done`, `delete`, and `edit`. Contributions can include bug fixes, new features, documentation improvements, or performance enhancements. Before contributing, please:

1. Check the GitHub Issues for existing tasks or report a new issue.
2. Discuss major changes or new features in an issue to align with the project’s goals.
3. Fork the repository and create a branch for your changes.

## Development Setup

To contribute to `clodo`, set up your local development environment as follows:

### Prerequisites

- **Rust and Cargo:** Install Rust via rustup:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

  Ensure you have Rust 1.56 or later:
  
  ```bash
  rustc --version
  ```

- **Git:** Installed and configured for GitHub access.
A code editor (e.g., VS Code with the `rust-analyzer` extension).

### Cloning the Repository

1. Fork the repository on GitHub.
2. Clone your fork:

  ```bash
  git clone https://github.com/fobabs/clodo.git
  cd clodo
  ```

3. Add the upstream repository:

  ```bash
  git remote add upstream https://github.com/fobabs/clodo.git
  ```

### Building and Testing

1. Build the project:

  ```bash
  cargo build
  ```

2. Run tests:

  ```bash
  cargo test
  ```

3. Try the CLI locally:

  ```bash
  cargo run -- add "Test task"
  cargo run -- list
  ```

### Formatting and Linting

- Format code with rustfmt:

  ```bash
  cargo fmt
  ```

- Run Clippy for linting:

  ```bash
  cargo clippy -- -D warnings
  ```

- Ensure all checks pass before submitting changes:

  ```bash
  cargo fmt -- --check
  cargo clippy -- -D warnings
  cargo test
  ```

## Contribution Guidelines

To ensure high-quality contributions, please follow these guidelines:

### Coding Standards

- Follow Rust’s idiomatic style as enforced by `rustfmt`.
- Write clear, concise, and documented code. Use doc comments (`///`) for public APIs (if applicable).
- Avoid introducing new dependencies unless necessary and justified.
- Ensure backward compatibility for CLI commands and JSON storage format.

### Writing Tests

- Add tests for new features or bug fixes in `src/models.rs` or `src/storage.rs` using Rust’s testing framework.
- Ensure 100% test coverage for new code where possible.

### Documentation

- Update `README.md` for new features or CLI command changes.
- Add or update doc comments for code changes.
- Generate and preview documentation:

  ```bash
  cargo doc --open
  ```

### Commit Messages

- Write clear, descriptive commit messages.
- Use the present tense (e.g., “Add feature” not “Added feature”).
- Reference issue numbers if applicable (e.g., “Fix #42: Handle empty todo list”).
- Example:

  ```
  Add edit task command
  - Implement edit_task in TodoList
  - Update CLI to support edit command
  - Add tests for editing tasks
  Closes #123
  ```

## Submitting Changes

1. **Create a Branch:**

- Use a descriptive branch name (e.g., `feature/add-priority`, `fix/bug-42`):

  ```bash
  git checkout -b feature/your-feature
  ```

2. **Make Changes:**

- Implement your changes, following the coding and testing guidelines.
- Run all checks (`cargo fmt`, `cargo clippy`, `cargo test`).

3. **Commit Changes:**

  ```bash
  git add .
  git commit -m "Add your feature"
  ```

4. **Push to Your Fork:**

  ```bash
  git push origin feature/your-feature
  ```

5. **Open a Pull Request:**

- Go to your fork on GitHub and create a pull request against the `main` branch of the upstream repository.
- Include a clear description of the changes, referencing any related issues.

6. **Address Feedback:**

- Respond to review comments and make requested changes.
- Push updates to the same branch to update the PR.

7. **CI Checks:**

- Ensure the GitHub Actions CI workflow (`.github/workflows/test.yml`) passes, including tests, formatting, and Clippy.

## Licensing

`clodo` is dual-licensed under the MIT License or Apache License 2.0, at the user’s option. By contributing to `clodo`, you agree that your contributions will be licensed under both the MIT License and Apache License 2.0. This ensures compatibility with the project’s dual-licensing model.

## Code of Conduct

We are committed to fostering an open and welcoming community. All contributors are expected to adhere to the Rust Code of Conduct. Please report any unacceptable behavior to the maintainer.

## Contact

For questions or discussions, please:

- Open an issue on GitHub.
- Contact the maintainer via GitHub (fobabs).

Thank you for contributing to clodo!
