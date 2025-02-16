//pub use self::{Error, Result};

use serde::Deserialize;
use tokio::net::TcpListener;
use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    // this composes multiple routes together
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());
        
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    println!("->> Routes configured: /hello and /hello2/{{name}}");
    
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().fallback_service(ServeDir::new("./"))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))

}

async fn handler_hello(Query(params):Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello {name}"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello {name}"))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}