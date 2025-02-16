use serde::Deserialize;
// use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    println!("->> Routes configured: /hello and /hello2/{{name}}");
    
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
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