use std::collections::HashMap;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

pub struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: String, description: String) -> u32 {
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

    pub fn update_status(&mut self, id: u32, status: TaskStatus) -> Option<()> {
        self.tasks.get_mut(&id).map(|task| {
            task.status = status;
        })
    }

    pub fn list_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by_key(|task| task.id);
        tasks
    }
}
