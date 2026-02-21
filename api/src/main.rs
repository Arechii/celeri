use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Food {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/food", get(get_food));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let _ = axum::serve(listener, app).await;
}

async fn get_food() -> impl IntoResponse {
    let food = Food {
        name: "Pizza".to_string(),
    };

    (StatusCode::OK, Json(food))
}
