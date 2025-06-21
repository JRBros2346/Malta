use axum::{routing, Router};
use malta_core::{tracing::Level, Malta};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod views;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let state = Malta::open().await.unwrap();
    let app = Router::new()
        .route("/projects", routing::get(views::get_projects))
        .route("/projects", routing::post(views::create_project))
        .fallback(views::not_found)
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
