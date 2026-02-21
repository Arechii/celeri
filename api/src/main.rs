use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;
use std::io::Error;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, ToSchema)]
struct Food {
    name: String,
}

#[derive(OpenApi)]
#[openapi(paths(get_food))]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new()
        .route("/food", get(get_food))
        .merge(SwaggerUi::new("/swagger").url("/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await
}

#[utoipa::path(
    get,
    path = "/food",
    responses(
        (status = 200, description = "Get food", body = Food)
    )
)]
async fn get_food() -> impl IntoResponse {
    let food = Food {
        name: "Pizza".to_string(),
    };

    (StatusCode::OK, Json(food))
}
