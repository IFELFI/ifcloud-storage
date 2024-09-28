use axum::{
    routing::{get, post},
    Router,
};
use routes::issue::issue_session;
use std::{
    env,
    net::{IpAddr, SocketAddr},
};
use tokio::net::TcpListener;

mod layers;
mod routes;
mod services;

//#[tokio::main]
pub async fn run() {
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

    let addr = SocketAddr::from((
        env::var("HOST")
            .unwrap_or("127.0.0.1".to_string())
            .parse::<IpAddr>()
            .unwrap(),
        env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse::<u16>()
            .unwrap(),
    ));
    let server = TcpListener::bind(&addr).await.unwrap();

    // serve axum
    println!("REST server is running on http://{}", addr);
    axum::serve(server, router).await.unwrap();
}
