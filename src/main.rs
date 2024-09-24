use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use futures::{stream::Next, StreamExt};
use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};
use routes::issue::issue_session;
use serde::Deserialize;
use services::{file_manager_service::ModifyFile, FileManagerService};
use std::{
    env,
    net::{IpAddr, SocketAddr},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

mod layers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    // load .env file
    dotenv().ok();

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

    // serve microservice
    //println!("Microservice is running on http://{}", addr);
    //tokio::spawn(microservice());

    // serve rabbit
    tokio::spawn(rabbit());

    // serve axum
    println!("Axum is running on http://{}", addr);
    axum::serve(server, router).await.unwrap();
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct MergeRequestData {
    file_key: String,
    total_chunk: u32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct DeleteRequestData {
    file_key: String,
}

#[derive(Deserialize)]
enum MicroRequestData {
    Merge(MergeRequestData),
    Delete(DeleteRequestData),
}

#[derive(Deserialize)]
struct RequestPattern {
    target: String,
    cmd: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct MicroRequest {
    pattern: RequestPattern,
    data: MicroRequestData,
}

enum RabbitResponseStatus {
    Success,
    Fail,
}

struct RabbitResponse {
    status: RabbitResponseStatus,
    message: String,
}

async fn rabbit() -> Result<()> {
    let addr = env::var("RABBIT_ADDR").unwrap_or_else(|_| "amqp://localhost:5672".to_string());
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let connection = Connection::connect(&addr, options).await?;
    let channel = connection.create_channel().await?;

    let _queue = channel.queue_declare(
        "storage",
        QueueDeclareOptions::default(),
        FieldTable::default(),
    );

    let consumer = channel
        .basic_consume(
            "storage",
            "storage",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        let delivery = match delivery {
            Ok(Some(delivery)) => delivery,
            Ok(None) => return,
            Err(error) => {
                dbg!("Failed to consume queue message {}", error);
                return;
            }
        };

        delivery
            .ack(BasicAckOptions::default())
            .await
            .expect("Failed to act send_webhook_event message");
    });

    Ok(())
}
