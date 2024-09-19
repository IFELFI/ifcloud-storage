use axum::{
    routing::{get, post},
    Router,
};
use routes::issue::issue_session;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod layers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let session_layer = layers::set_session_layer().await.unwrap();

    let index_routes = Router::new().route("/", get(|| async { "Hello, world!" }));

    let file_manage_routes = Router::new()
        .route("/read", get(routes::file::read))
        .route("/write", post(routes::file::write));

    let issue_session_routes = Router::new().route("/issue/:token", get(issue_session));

    let router = Router::new()
        .nest("/", index_routes)
        .nest("/file", file_manage_routes)
        .nest("/session", issue_session_routes)
        .layer(session_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp, router).await.unwrap();
}
