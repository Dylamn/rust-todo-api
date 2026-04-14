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

pub async fn list_tasks(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Task>>, ApiError> {
    let tasks = task_service::list_tasks(&state.db).await?;
    Ok(Json(tasks))
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TaskCreate>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    let new_task = task_service::create_task(&state.db, payload.description).await?;

    Ok((StatusCode::CREATED, Json(new_task)))
}

pub async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(ident): Path<Uuid>,
    Json(payload): Json<TaskUpdate>,
) -> Result<Json<Arc<Task>>, ApiError> {
    task_service::find_task_by_id(&state.db, ident).await?;
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
