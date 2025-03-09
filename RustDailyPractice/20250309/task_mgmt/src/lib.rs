// define the module
pub mod task {
    #[derive(Debug)]
    pub struct Task {
        pub id: u32,
        pub title: String,
        pub description: String,
        pub completed: bool
    }

    impl Task {
        // constructor to create new task
        pub fn new(id: u32, title: &str, description: &str) -> Self {
            Self {
                id,
                title: title.to_string(),
                description: description.to_string(),
                completed: false
            }
        }

        // mark task as completed
        pub fn complete(&mut self) {
            self.completed = true;
        }

        // update title and description
        pub fn update(&mut self, title: &str, description: &str) {
            self.title = title.to_string();
            self.description = description.to_string();
        }
    }

    // function to delete a task
    pub fn delete(task_id: u32) {
        println!("Deleting task {}", task_id);
    }
}

// the list module
pub mod list {
    use crate::task::Task;

    pub struct TaskList {
        tasks: Vec<Task>,
    }

    impl TaskList {
        // constructor
        pub fn new() -> Self {
            Self { tasks: Vec::new() }
        }

        // add a task to the list
        pub fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        // remove a task on the list by id
        pub fn remove_task(&mut self, id: u32) {
            self.tasks.retain(|task| task.id != id);
        }

        // retrieve a task by id
        pub fn get_task(&self, id: u32) -> Option<&Task> {
            self.tasks.iter().find(|task| task.id == id)
        }

        // list all the tasks
        pub fn list_tasks(&self) {
            for task in &self.tasks {
                println!("{:?}", task);
            }
        }
    }
}