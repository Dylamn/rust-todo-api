use super::models::task::Task;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub tasks: RwLock<Vec<Arc<Task>>>,
}

pub fn init_state() -> Arc<AppState> {
    let state = AppState {
        tasks: RwLock::new(vec![]),
    };

    Arc::new(state)
}
