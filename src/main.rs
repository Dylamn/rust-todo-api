use anyhow::Context;
use axum::Router;
use std::sync::Arc;
use todo_api::state::AppState;
use todo_api::{db, routes, state};

const DATABASE_URL: &str = "sqlite://database.db";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = db::init_db(DATABASE_URL).await?;
    let app_state = state::init_state(db);
    let app = app_factory(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .context("Error running HTTP server")
}

fn app_factory(state: Arc<AppState>) -> Router {
    Router::new().merge(routes::tasks::get_router(state))
}
