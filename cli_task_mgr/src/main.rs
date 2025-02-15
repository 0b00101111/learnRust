use std::collections::HashMap;
use std::io::{self, Write};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum TaskStatus {
    Pending,
    InProgress,
    Complete,
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
}
