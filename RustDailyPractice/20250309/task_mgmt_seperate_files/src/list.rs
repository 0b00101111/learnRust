use crate::task::Task;

pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("{:?}", task);
        }
    }
}
