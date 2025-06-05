use serde::{Deserialize, Serialize};

/// A todo item
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    /// The ID of the todo item
    pub id: u32,
    /// The task of the todo item
    pub task: String,
    /// Whether the todo item is completed
    pub completed: bool,
}

/// A collection of todo items
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    /// A vector of todo items
    pub todos: Vec<Todo>,
    /// The ID of the next todo item to be created
    pub next_id: u32,
}

impl TodoList {
    /// Creates a new TodoList with an empty vector of todos and a next_id of 1.
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            next_id: 1,
        }
    }


    /// Adds a new todo item to the list.
    ///
    /// The new todo item is assigned an ID of `self.next_id` and is marked as not completed.
    /// The `next_id` is then incremented.
    ///
    pub fn add_task(&mut self, task: String) {
        self.todos.push(Todo {
            id: self.next_id,
            task,
            completed: false,
        });
        self.next_id += 1
    }

    /// Marks the todo item with the given ID as completed.
    ///
    /// This function searches for a todo item with the specified ID in the list.
    /// If found, it sets the item's `completed` status to `true`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the todo item to mark as completed.
    ///
    /// # Returns
    ///
    /// `true` if the todo item was found and marked as completed, otherwise `false`.

    pub fn mark_done(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            return true;
        }
        false
    }

    /// Deletes the todo item with the given ID from the list.
    ///
    /// This function searches for a todo item with the specified ID in the list.
    /// If found, it removes the item from the list.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the todo item to delete.
    ///
    /// # Returns
    ///
    /// `true` if the todo item was found and deleted, otherwise `false`.
    pub fn delete_task(&mut self, id: u32) -> bool {
        if let Some(index) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(index);
            return true;
        }
        false
    }

    /// Edits the task of the todo item with the given ID.
    ///
    /// This function searches for a todo item with the specified ID in the list.
    /// If found, it sets the item's task to the new value.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the todo item to edit.
    /// * `task` - The new task of the todo item.
    ///
    /// # Returns
    ///
    /// `true` if the todo item was found and edited, otherwise `false`.
    pub fn edit_task(&mut self, id: u32, task: String) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.task = task;
            return true;
        }
        false
    }
}
