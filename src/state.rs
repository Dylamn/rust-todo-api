use super::models::task::Task;
use std::sync::Arc;
use sqlx::{Pool, Sqlite};
use tokio::sync::RwLock;

pub struct AppState {
    pub tasks: RwLock<Vec<Arc<Task>>>,
    pub db: Pool<Sqlite>,
}

pub fn init_state(db: Pool<Sqlite>) -> Arc<AppState> {
    let state = AppState {
        tasks: RwLock::new(vec![]),
        db
    };

    Arc::new(state)
}
