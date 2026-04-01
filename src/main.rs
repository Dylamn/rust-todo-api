pub mod models;
pub mod routes;
mod state;

use axum::{Router, routing::get};

use routes::tasks;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .merge(tasks::get_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
