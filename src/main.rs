mod transform;

use axum::{
    extract::Json,
    http::Method,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct TransformRequest {
    text: String,
}

#[derive(Serialize)]
struct TransformResponse {
    original: String,
    transformed: String,
}

async fn handle_transform(Json(req): Json<TransformRequest>) -> Json<TransformResponse> {
    let transformed = transform::transform(&req.text);
    Json(TransformResponse {
        original: req.text,
        transformed,
    })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/transform", post(handle_transform))
        .fallback_service(ServeDir::new("frontend/dist"))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
