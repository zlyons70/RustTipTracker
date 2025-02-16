use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};

pub fn routes() -> Router {
    Router::new().route("/api/login",post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:1<12} - api_login", "HANDLER");
    // TODO: Implement 
    if payload.username != "demo1" || payload.pwd != "demo1" {
        return Err(Error::LoginFail);
    }

    // TODO: Set Cookies

    // Create success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    todo!()
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}