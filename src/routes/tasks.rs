use crate::state::{AppState, init_state};
use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;
use std::sync::Arc;
use crate::models::Task;

pub fn get_router() -> Router {
    Router::new()
        .route("/tasks", get(index))
        .with_state(init_state())
}

async fn index(State(state): State<Arc<AppState>>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();

    Json(tasks.clone())
}
