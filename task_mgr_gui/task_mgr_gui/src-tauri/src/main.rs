// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod task_manager;
use task_manager::{Task, TaskManager, TaskStatus};
use std::sync::Mutex;
use tauri::State;

struct TaskManagerState(Mutex<TaskManager>);

#[tauri::command]
fn add_task(state: State<TaskManagerState>, title: String, description: String) -> Result<u32, String> {
    let mut manager = state.0.lock().unwrap();
    Ok(manager.add_task(title, description))
}

#[tauri::command]
fn list_tasks(state: State<TaskManagerState>) -> Result<Vec<Task>, String> {
    let manager = state.0.lock().unwrap();
    Ok(manager.list_tasks().into_iter().cloned().collect())
}

#[tauri::command]
fn update_status(state: State<TaskManagerState>, id: u32, status: TaskStatus) -> Result<(), String> {
    let mut manager = state.0.lock().unwrap();
    manager.update_status(id, status)
        .ok_or_else(|| "Task not found".to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(TaskManagerState(Mutex::new(TaskManager::new())))
        .invoke_handler(tauri::generate_handler![
            add_task,
            list_tasks,
            update_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
