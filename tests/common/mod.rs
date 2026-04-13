#![allow(dead_code)]
use axum_test::TestServer;
use std::sync::Arc;
use todo_api::db;
use todo_api::models::Task;
use todo_api::routes::tasks::get_router;
use todo_api::services::task_service;
use todo_api::state::{init_state, AppState};
use tokio::sync::RwLock;

pub async fn setup() -> TestServer {
    TestServer::new(get_router(init_state(
        db::init_db("sqlite::memory:").await.unwrap(),
    )))
}

pub fn setup_with_state(state: AppState) -> TestServer {
    let state = Arc::new(state);
    TestServer::new(get_router(state))
}

pub async fn populate_state_with_tasks(descriptions: Vec<&str>) -> AppState {
    let tasks = descriptions
        .iter()
        .map(|description| Arc::new(task_service::create_task(description.to_string()).unwrap()))
        .collect::<Vec<Arc<Task>>>();

    AppState {
        tasks: RwLock::new(tasks),
        db: db::init_db("sqlite::memory:").await.unwrap(),
    }
}

pub async fn create_task_in_state(description: &str) -> (AppState, uuid::Uuid) {
    let task = task_service::create_task(description.to_string()).unwrap();
    let id = task.id;
    let state = AppState {
        tasks: RwLock::new(vec![Arc::new(task)]),
        db: db::init_db("sqlite::memory:").await.unwrap(),
    };
    (state, id)
}
