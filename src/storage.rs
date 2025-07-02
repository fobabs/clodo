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
    let serialized = serde_json::to_string_pretty(&todo_list)?;
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Todo;
    use tempfile::TempDir;

    #[test]
    fn test_load_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let todo_list = load_todos().unwrap();
        assert_eq!(todo_list.todos.len(), 0);
        assert_eq!(todo_list.next_id, 1);

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_save_and_load_todos() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let todo_list = TodoList {
            todos: vec![Todo {
                id: 1,
                task: "Test task".to_string(),
                completed: false,
            }],
            next_id: 2,
        };
        save_todos(&todo_list).unwrap();

        let loaded = load_todos().unwrap();
        assert_eq!(loaded, todo_list);

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_load_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let path = Path::new("todos.json");
        let mut file = File::create(path).unwrap();
        file.write_all(b"invalid json").unwrap();

        let result = load_todos();
        assert!(result.is_err());

        std::env::set_current_dir(original_dir).unwrap();
    }
}
