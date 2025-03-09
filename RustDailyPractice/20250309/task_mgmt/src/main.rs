use task_mgmt::task::Task;
use task_mgmt::list::TaskList;

fn main() {
    let mut task_list = TaskList::new();

    let task1 = Task::new(1, "Learn Rust", "Practice Rust modules and structs");
    let task2 = Task::new(2, "Write Code", "Implement a task manager in Rust");

    task_list.add_task(task1);
    task_list.add_task(task2);

    task_list.list_tasks();

    if let Some(task) = task_list.get_task(1) {
        println!("Found task: {}", task.id);
    }

    task_list.remove_task(1);
    task_list.list_tasks();
}