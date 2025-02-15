use std::collections::HashMap;
use std::io::{self, Write};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: TaskStatus,
    created_at: DateTime<Local>,
}

struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self{
        Self {
            tasks: HashMap::new(),
            next_id: 1,

        }
    }

    fn add_task(&mut self, title: String, description: String) -> u32 {
        let id = self.next_id;
        let task = Task {
            id,
            title,
            description,
            status: TaskStatus::Pending,
            created_at: Local::now(),
        };
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }

    fn update_status(&mut self, id: u32, status: TaskStatus) -> Option<()> {
        self.tasks.get_mut(&id).map(|task| {
            task.status = status;
        })
    }

    fn list_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by_key(|task| task.id);
        tasks
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(filename, json)
    }

    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let content = fs::read_to_string(filename)?;
        self.tasks = serde_json::from_str(&content)?;
        self.next_id = self.tasks.keys().max().map_or(1, |&id| id + 1);
        Ok(())
    }
}

fn main() {
    let mut manager = TaskManager::new();

    if let Err(e) = manager.load_from_file("task.json") {
        println!("No existing tasks found or error loading file: {}", e);
    }

    loop {
        println!("\nTask Manager - Available Commands:");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Update task status");
        println!("4. Save and exit");

        println!("Enter command (1-4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                print!("Enter task title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                let id = manager.add_task(title.trim().to_string(),
                                        description.trim().to_string());
                println!("Task added with ID: {}", id);
            }

            "2" => {
                println!("\nCurrent Tasks:");
                for task in manager.list_tasks() {
                    println!("ID: {}", task.id);
                    println!("Title: {}", task.title);
                    println!("Description: {}", task.description);
                    println!("Status: {:?}", task.status);
                    println!("Created: {}",
                             task.created_at.format("%Y-%m-%d %H-%M-%S"));
                    println!("---");
                }
            }

            "3" => {
                print!("Enter task ID: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let id: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid ID");
                        continue;
                    }
                };

                println!("select new status:");
                println!("1. Pending");
                println!("2. In Progress:");
                println!("3. Completed");

                print!("Enter status (1-3): ");
                io::stdout().flush().unwrap();
                let mut status_input = String::new();
                io::stdin().read_line(&mut status_input).unwrap();

                let new_status = match status_input.trim() {
                    "1" => TaskStatus::Pending,
                    "2" => TaskStatus::InProgress,
                    "3" => TaskStatus::Completed,
                    _ => {
                        println!("Invalid status");
                        continue;
                    }
                };

                match manager.update_status(id, new_status) {
                    Some(_) => println!("Task status updated"),
                    None => println!("Task not found"),
                }
            }

            "4" => {
                if let Err(e) = manager.save_to_file("task.json") {
                    println!("Error saving tasks: {}", e);
                } else {
                    println!("Tasks saved successfully");
                }
                break;
            }

            _ => println!("Invalid command"),
        }
    }
}
