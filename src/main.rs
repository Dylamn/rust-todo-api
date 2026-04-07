use axum::{Router, routing::get};
use std::sync::Arc;

use todo_api::state::AppState;
use todo_api::{routes::tasks, state};

#[tokio::main]
async fn main() {
    let app_state = state::init_state();
    let app = application(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn application(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .merge(tasks::get_router(state))
}
