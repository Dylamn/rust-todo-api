use super::models::task::Task;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub tasks: Mutex<Vec<Task>>,
}

pub fn init_state() -> Arc<AppState> {
    let state = AppState {
        tasks: Mutex::new(vec![Task::new(1, "A static task".to_string())]),
    };

    Arc::new(state)
}
