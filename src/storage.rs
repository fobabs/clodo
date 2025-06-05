use crate::models::TodoList;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

/// Loads the TodoList from a JSON file named "todos.json". If the file does not exist,
/// an empty TodoList is returned.
pub fn load_todos() -> Result<TodoList, Box<dyn std::error::Error>> {
    let path = Path::new("todos.json");
    if !path.exists() {
        return Ok(TodoList::new());
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
pub fn save_todos(todo_list: &TodoList) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("todos.json");
    let serialized = serde_json::to_string(&todo_list)?;
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
