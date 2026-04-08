use axum::http::StatusCode;
use axum_test::expect_json;
use serde_json::json;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn should_mark_task_as_done() {
    let (state, id) = common::create_task_in_state("A task to complete");
    let server = common::setup_with_state(state);

    let response = server
        .put(&format!("/tasks/{id}"))
        .json(&json!({ "done": true }))
        .await;

    response
        .assert_status(StatusCode::OK)
        .assert_json(&json!({
            "id": expect_json::uuid(),
            "description": "A task to complete",
            "status": "Completed",
            "created_at": expect_json::string(),
            "completed_at": expect_json::string(),
        }));
}

#[tokio::test]
async fn should_mark_task_as_not_done_after_completion() {
    let (state, id) = common::create_task_in_state("A task to reopen");
    let server = common::setup_with_state(state);

    server
        .put(&format!("/tasks/{id}"))
        .json(&json!({ "done": true }))
        .await;

    let response = server
        .put(&format!("/tasks/{id}"))
        .json(&json!({ "done": false }))
        .await;

    response
        .assert_status(StatusCode::OK)
        .assert_json(&json!({
            "id": expect_json::uuid(),
            "description": "A task to reopen",
            "status": "Pending",
            "created_at": expect_json::string(),
            "completed_at": null,
        }));
}

#[tokio::test]
async fn should_return_not_found_for_unknown_task() {
    let server = common::setup();
    let unknown_id = Uuid::new_v4();

    let response = server
        .put(&format!("/tasks/{unknown_id}"))
        .json(&json!({ "done": true }))
        .await;

    response
        .assert_status(StatusCode::NOT_FOUND)
        .assert_json(&json!({
            "error": {
                "code": "not_found",
                "message": "Resource not found",
            }
        }));
}
