#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: &str, description: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn update(&mut self, title: &str, description: &str) {
        self.title = title.to_string();
        self.description = description.to_string();
    }
}
