use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn should_delete_existing_task() {
    let (state, id) = common::create_task_in_state("Task to delete").await;
    let server = common::setup_with_state(state);

    let response = server.delete(&format!("/tasks/{id}")).await;

    response.assert_status(StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn should_return_204_for_non_existent_task() {
    let server = common::setup().await;
    let unknown_id = Uuid::new_v4();

    let response = server.delete(&format!("/tasks/{unknown_id}")).await;

    response.assert_status(StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn should_remove_task_from_list_after_deletion() {
    let (state, id) = common::create_task_in_state("Task to remove").await;
    let server = common::setup_with_state(state);

    server.delete(&format!("/tasks/{id}")).await;

    let response = server.get("/tasks").await;

    response
        .assert_status(StatusCode::OK)
        .assert_json(&json!([]));
}
