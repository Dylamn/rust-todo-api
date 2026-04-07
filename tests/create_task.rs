use axum::http::StatusCode;
use axum_test::{expect_json, TestServer};
use todo_api::routes::tasks::get_router;
use todo_api::state::init_state;

use serde_json::json;

#[tokio::test]
async fn test_api_should_create_task() {
    let state = init_state();
    let server = TestServer::new(get_router(state));

    let task_desc = "My first task";

    let payload = json!({ "description": task_desc });
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
