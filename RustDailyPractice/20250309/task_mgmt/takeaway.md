# Case 1: Modules Defined Inside lib.rs
When all modules are inside lib.rs, you need to use the use keyword with the crate name to bring them into main.rs.

Example:

src/lib.rs (defines modules inside the library crate):

```rust
pub mod task {
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
    }
}
```

src/main.rs (brings modules from the library crate into scope):

```rust
use task_mgmt::task::Task; // `task_mgmt` is the crate name

fn main() {
    let my_task = Task::new(1, "Learn Rust", "Practice modules");
    println!("Task created: {}", my_task.title);
}
```

# Case 2: Modules in Separate Files

When each module is in its own file, main.rs can use mod to include them without the use keyword.

Example:

src/task.rs (module in its own file):

```rust
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
}
```
src/main.rs (includes task.rs):

```rust
mod task;  // Includes the module (no `use` needed)

use task::Task; // Brings Task into scope

fn main() {
    let my_task = Task::new(1, "Learn Rust", "Practice modules");
    println!("Task created: {}", my_task.title);
}
```

# Key points:
- mod task; tells Rust to look for task.rs in src/. 
- task::Task; is needed to bring the struct into scope.
# When to Choose Which?
- ✅ Keep modules in lib.rs if your project is small.
- ✅ Use separate files (task.rs, list.rs) when:
  - The project grows larger.
  -	You want cleaner and better-organized code.