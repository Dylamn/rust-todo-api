use crate::handlers::tasks;
use crate::state::AppState;
use axum::routing::{Router, delete, get, post, put};
use std::sync::Arc;

pub fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/tasks", get(tasks::list_tasks))
        .route("/tasks", post(tasks::create_task))
        .route("/tasks/{ident}", put(tasks::update_task))
        .route("/tasks/{ident}", delete(tasks::destroy_task))
        .with_state(state)
}
