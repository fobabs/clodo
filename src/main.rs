mod models;
mod storage;

use clap::{Parser, Subcommand};
use colored::Colorize;
use models::TodoList;
use storage::{load_todos, save_todos};

#[derive(Parser)]
#[command(name = "clodo", version = "0.1.0", about = "A simple todo list manager", long_about = None)]
/// Command line interface for the todo list manager
struct Cli {
    #[command(subcommand)]
    /// The action to perform
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
    Edit { id: u32, title: String },
}

/// The entry point of the program
///
/// This function parses the command line arguments passed to the program and
/// performs the desired action. The actions are:
///
/// - Add a new todo item
/// - List all todo items
/// - Mark a todo item as done
/// - Delete a todo item
/// - Edit a todo item
///
/// The function also saves the state of the todo list to a file after each action.
fn main() {
    let cli = Cli::parse();
    let mut todo_list = load_todos().unwrap_or_else(|_| TodoList::new());

    match cli.commands {
        Commands::Add { title } => {
            todo_list.add_task(title);
            println!("{}", "Task added successfully!".green());
        }
        Commands::List => {
            if todo_list.todos.is_empty() {
                println!("{}", "No todos found.".yellow());
                return;
            }
            println!("{}", "Todos:".bold());
            for todo in &todo_list.todos {
                let status = if todo.completed {
                    "✔ Done".green()
                } else {
                    "⬜ Pending".red()
                };
                println!("{}: {} ({})", todo.id, todo.title, status);
            }
        }
        Commands::Done { id } => {
            if todo_list.mark_done(id) {
                println!("{}", "Task marked as done!".green());
            } else {
                println!("{}", "Task not found.".red());
            }
        }
        Commands::Delete { id } => {
            if todo_list.delete_task(id) {
                println!("{}", "Task deleted successfully!".green());
            } else {
                println!("{}", "Task not found.".red());
            }
        }
        Commands::Edit { id, title } => {
            if todo_list.edit_task(id, title) {
                println!("{}", "Task edited successfully!".green());
                save_todos(&todo_list).expect("Failed to save todos");
            } else {
                println!("{}", "Task not found.".red());
            }
        }
    }

    save_todos(&todo_list).expect("Failed to save todos");
}
