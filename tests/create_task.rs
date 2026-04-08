use axum::http::StatusCode;
use axum_test::expect_json;

use serde_json::json;

mod common;

fn task_payload(description: &str) -> serde_json::Value {
    json!({ "description": description })
}

#[tokio::test]
async fn test_api_should_create_task() {
    let server = common::setup();

    let task_desc = "My first task";

    let payload = task_payload(&task_desc);
    let response = server.post("/tasks").json(&payload).await;

    response
        .assert_status(StatusCode::CREATED)
        .assert_json(&json!({
            "id": expect_json::uuid(),
            "description": task_desc,
            "status": "Pending",
            "created_at": expect_json::string(),
            "completed_at": null,
        }));
}

#[tokio::test]
async fn test_api_should_not_create_task_with_empty_description() {
    let server = common::setup();

    let payload = task_payload("");

    let response = server.post("/tasks").json(&payload).await;

    response
        .assert_status(StatusCode::UNPROCESSABLE_ENTITY)
        .assert_json(&json!({
            "error": {
                "code": "task_description_empty",
                "message": "The description cannot be empty",
            }
        }));
}

#[tokio::test]
async fn should_not_create_task_with_too_long_description() {
    let server = common::setup();
    let long_description = "a".repeat(101);

    let response = server.post("/tasks").json(&task_payload(&long_description)).await;

    response
        .assert_status(StatusCode::UNPROCESSABLE_ENTITY)
        .assert_json(&json!({
            "error": {
                "code": "task_description_too_long",
                "message": "The description cannot be longer than 100 bytes",
            }
        }));
}

#[tokio::test]
async fn should_not_create_task_with_whitespace_only_description() {
    let server = common::setup();

    let response = server.post("/tasks").json(&task_payload("   ")).await;

    response
        .assert_status(StatusCode::UNPROCESSABLE_ENTITY)
        .assert_json(&json!({
            "error": {
                "code": "task_description_empty",
                "message": "The description cannot be empty",
            }
        }));
}

#[tokio::test]
async fn should_trim_description_whitespace() {
    let server = common::setup();

    let response = server.post("/tasks").json(&task_payload("  hello  ")).await;

    response
        .assert_status(StatusCode::CREATED)
        .assert_json(&json!({
            "id": expect_json::uuid(),
            "description": "hello",
            "status": "Pending",
            "created_at": expect_json::string(),
            "completed_at": null,
        }));
}
