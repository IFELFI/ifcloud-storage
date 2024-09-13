use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod routes;
mod layers;
mod services;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(|| async { "Hello, World!" })).route("/file/read", get(routes::file::read::read));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp, router).await.unwrap();
}
