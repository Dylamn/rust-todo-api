use axum::http::StatusCode;
use axum_test::expect_json;
use serde_json::json;
use todo_api::models::Status;

mod common;

#[tokio::test]
async fn should_return_empty_list_when_no_tasks() {
    let server = common::setup();

    let response = server.get("/tasks").await;

    response
        .assert_status(StatusCode::OK)
        .assert_json(&json!([]));
}

#[tokio::test]
async fn should_list_all_created_tasks() {
    let state = common::populate_state_with_tasks(vec!["Learn Rust", "Learn Python"]);
    let server = common::setup_with_state(state);

    let response = server.get("/tasks").await;

    response.assert_status(StatusCode::OK).assert_json(&json!([
        {
            "id": expect_json::uuid(),
            "description": "Learn Rust",
            "status": Status::Pending,
            "created_at": expect_json::string(),
            "completed_at": null,
        },
        {
            "id": expect_json::uuid(),
            "description": "Learn Python",
            "status": Status::Pending,
            "created_at": expect_json::string(),
            "completed_at": null,
        },
    ]));
}
