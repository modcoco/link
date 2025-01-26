use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|| async {
            let message = Message {
                message: "Hello, World!".to_string(),
            };
            Json(message)
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
