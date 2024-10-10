use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use routes::session::{delete_session, issue_session};
use std::{
    env,
    net::{IpAddr, SocketAddr},
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod layers;
mod routes;
mod services;

pub async fn run() {
    // *************
    // Layers
    // *************

    // Cors layer
    // Allowed origins
    let allowed_origins = ["http://localhost:3005".parse().unwrap()];
    let cors_layer = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true);

    // Session layer
    let session_layer = layers::set_session_layer().await.unwrap();

    // *************
    // Routes
    // *************

    // Index routes
    let index_routes = Router::new().route("/", get(|| async { "Hello, world!" }));

    // File manage routes
    let file_manage_routes = Router::new()
        .route("/:file_key", get(routes::file::read))
        .route("/:file_key", post(routes::file::write));

    // Session routes
    let session_routes = Router::new()
        .route("/issue/:token", get(issue_session))
        .route("/delete/:token", post(delete_session));

    // *************
    // Build server
    // *************

    // Router
    let router = Router::new()
        .nest("/", index_routes)
        .nest("/file", file_manage_routes)
        .nest("/session", session_routes)
        // cors for localhost:3005
        .layer(cors_layer)
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
