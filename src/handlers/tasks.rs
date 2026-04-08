use crate::error::ApiError;
use crate::models::Task;
use crate::services::task_service;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct TaskCreate {
    description: String,
}

#[derive(Deserialize)]
pub(crate) struct TaskUpdate {
    done: bool,
}

pub async fn list_tasks(State(state): State<Arc<AppState>>) -> Json<Vec<Arc<Task>>> {
    let tasks = state.tasks.read().await;

    Json(tasks.clone())
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TaskCreate>,
) -> Result<(StatusCode, Json<Arc<Task>>), ApiError> {
    let new_task = Arc::new(task_service::create_task(payload.description)?);

    {
        let mut tasks = state.tasks.write().await;
        tasks.push(new_task.clone());
    }

    Ok((StatusCode::CREATED, Json(new_task)))
}

pub async fn update_task(
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
        task_service::mark_task_done(&mut new_task);
    } else {
        task_service::mark_task_pending(&mut new_task);
    }

    let new_task = Arc::new(new_task);
    tasks[pos] = Arc::clone(&new_task);

    Ok(Json(new_task))
}

pub async fn destroy_task(
    State(state): State<Arc<AppState>>,
    Path(ident): Path<Uuid>,
) -> StatusCode {
    {
        let mut tasks = state.tasks.write().await;

        if let Some(position) = tasks.iter().position(|t| t.id == ident) {
            tasks.remove(position);
        }
    }

    StatusCode::NO_CONTENT
}
