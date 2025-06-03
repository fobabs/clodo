use std::{
    fs::File, 
    io::{Read, Write}, 
    path::Path
};
use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};


/// A todo item
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    /// The ID of the todo item
    id: u32,
    /// The title of the todo item
    title: String,
    /// Whether the todo item is completed
    completed: bool,
}

/// A collection of todo items
#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    /// A vector of todo items
    todos: Vec<Todo>,
    /// The ID of the next todo item to be created
    next_id: u32
}

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
    Edit { id: u32, task: String }
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
    let mut todo_list = load_todos().unwrap_or_else(|_| TodoList { todos: Vec::new(), next_id: 1 });

    match cli.commands {
        Commands::Add { title } => {
            todo_list.todos.push(Todo { id: todo_list.next_id, title, completed: false });
            println!("{}", "Task added successfully!".green());
            todo_list.next_id += 1;
            save_todos(&todo_list).expect("Failed to save todos");
        }
        Commands::List => {
            if todo_list.todos.is_empty() {
                println!("{}", "No todos found.".yellow());
                return;
            }
            println!("{}", "Todos:".bold());
            for todo in &todo_list.todos {
                let status = if todo.completed { "✔ Done".green() } else { "⬜ Pending".red() };
                println!("{}: {} ({})", todo.id, todo.title, status);
            }
        }
        Commands::Done { id } => {
            if let Some(todo) = todo_list.todos.iter_mut().find(|todo| todo.id == id) {
                todo.completed = true;
                println!("{}", "Task marked as done!".green());
                save_todos(&todo_list).expect("Failed to save todos");
            } else {
                println!("{}", "Task not found.".red());
            }
        }
        Commands::Delete { id } => {
            if let Some(index) = todo_list.todos.iter().position(|todo| todo.id == id) {
                todo_list.todos.remove(index);
                println!("{}", "Task deleted successfully!".green());
                save_todos(&todo_list).expect("Failed to save todos");
            } else {
                println!("{}", "Task not found.".red());
            }
        }
        Commands::Edit { id, task } => {
            if let Some(todo) = todo_list.todos.iter_mut().find(|todo| todo.id == id) {
                todo.title = task;
                println!("{}", "Task edited successfully!".green());
                save_todos(&todo_list).expect("Failed to save todos");
            } else {
                println!("{}", "Task not found.".red());
            }
        }
    }
}

/// Loads the TodoList from a JSON file named "todos.json". If the file does not exist,
/// an empty TodoList is returned.
fn load_todos() -> Result<TodoList, Box<dyn std::error::Error>> {
    let path = Path::new("todos.json");
    if !path.exists() {
        return Ok(TodoList { todos: Vec::new(), next_id: 1 });
    }
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(serde_json::from_str(&contents)?)
}

/// Saves the given TodoList to a JSON file named "todos.json".
///
/// This function serializes the given TodoList to JSON and writes it to a file.
/// If the file already exists, it is overwritten. If the file does not exist,
/// it is created.
///
/// # Errors
///
/// This function returns an error if the file cannot be created or written to.
fn save_todos(todo_list: &TodoList) -> Result<(), Box<dyn std::error::Error>>{
    let path = Path::new("todos.json");
    let serialized = serde_json::to_string(&todo_list)?;
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
