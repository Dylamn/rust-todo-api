use axum_test::TestServer;
use std::sync::Arc;
use tokio::sync::RwLock;
use todo_api::models::Task;
use todo_api::routes::tasks::get_router;
use todo_api::services::task_service;
use todo_api::state::{init_state, AppState};

pub fn setup() -> TestServer {
    TestServer::new(get_router(init_state()))
}

pub fn setup_with_state(state: AppState) -> TestServer {
    let state = Arc::new(state);
    TestServer::new(get_router(state))
}

pub fn populate_state_with_tasks(descriptions: Vec<&str>) -> AppState {
    let tasks = descriptions
        .iter()
        .map(|description| Arc::new(task_service::create_task(description.to_string()).unwrap()))
        .collect::<Vec<Arc<Task>>>();

    AppState { tasks: RwLock::new(tasks) }
}

pub fn create_task_in_state(description: &str) -> (AppState, uuid::Uuid) {
    let task = task_service::create_task(description.to_string()).unwrap();
    let id = task.id;
    let state = AppState {
        tasks: RwLock::new(vec![Arc::new(task)]),
    };
    (state, id)
}
