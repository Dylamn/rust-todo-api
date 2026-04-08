use axum::{Router};
use std::sync::Arc;

use todo_api::state::AppState;
use todo_api::{routes, state};

#[tokio::main]
async fn main() {
    let app_state = state::init_state();
    let app = app_factory(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn app_factory(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(routes::tasks::get_router(state))
}
