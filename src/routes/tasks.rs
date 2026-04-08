use crate::error::ApiError;
use crate::models::{Status, Task};
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize)]
struct TaskCreate {
    description: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct TaskUpdate {
    done: bool,
}

pub fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/tasks", get(index))
        .route("/tasks", post(create))
        .route("/tasks/{ident}", put(update))
        .route("/tasks/{ident}", delete(destroy))
        .with_state(state)
}

#[axum::debug_handler]
async fn index(State(state): State<Arc<AppState>>) -> Json<Vec<Arc<Task>>> {
    let tasks = state.tasks.read().await;

    Json(tasks.clone())
}

#[allow(dead_code)]
async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TaskCreate>,
) -> Result<(StatusCode, Json<Arc<Task>>), ApiError> {
    let ident = Uuid::new_v4();
    let new_task = Arc::new(Task::new(ident, payload.description)?);

    {
        let mut tasks = state.tasks.write().await;
        tasks.push(new_task.clone());
    }

    Ok((StatusCode::CREATED, Json(new_task)))
}

#[allow(dead_code)]
async fn update(
    State(state): State<Arc<AppState>>,
    Path(ident): Path<Uuid>,
    Json(payload): Json<TaskUpdate>,
) -> Result<Json<Arc<Task>>, ApiError> {
    let mut tasks = state.tasks.write().await;
    let pos = tasks
        .iter()
        .position(|t| t.id == ident)
        .ok_or(ApiError::NotFound)?;

    let old_task = &tasks[pos];

    let mut new_task = (**old_task).clone();
    if payload.done {
        new_task.completed_at = Some(Utc::now());
        new_task.status = Status::Completed;
    } else {
        new_task.completed_at = None;
        new_task.status = Status::Pending;
    }

    let new_task = Arc::new(new_task);
    tasks[pos] = Arc::clone(&new_task);

    Ok(Json(new_task))
}

#[allow(dead_code)]
async fn destroy(State(state): State<Arc<AppState>>, Path(ident): Path<Uuid>) -> StatusCode {
    {
        let mut tasks = state.tasks.write().await;

        if let Some(position) = tasks.iter().position(|t| t.id == ident) {
            tasks.remove(position);
        }
    }

    StatusCode::NO_CONTENT
}
