//use crate::{Error, Result};
use axum::{debug_handler, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};

pub fn routes() -> Router {
    Router::new().route("/api/login",post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> impl IntoResponse {
    println!("->> {:1<12} - api_login", "HANDLER");
    // TODO: Implement 
    if payload.username != "demo1" || payload.pwd != "demo1" {
        return (axum::http::StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"})));
    }

    // TODO: Set Cookies

    // Create success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    (axum::http::StatusCode::OK, body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}