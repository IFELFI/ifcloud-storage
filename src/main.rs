use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, QueueDeclareOptions};
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
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
#[allow(dead_code)]
struct MicroRequest {
    cmd: String,
    data: MicroRequestData,
}

async fn microservice() -> Result<()> {
    let addr = SocketAddr::from((
        env::var("MICRO_HOST")
            .unwrap_or("127.0.0.1".to_string())
            .parse::<IpAddr>()
            .unwrap(),
        env::var("MICRO_PORT")
            .unwrap_or("3001".to_string())
            .parse::<u16>()
            .unwrap(),
    ));
    let server = TcpListener::bind(&addr).await?;

    loop {
        let (mut socket, _) = server.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n: MicroRequest = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        let json: MicroRequest = serde_json::from_slice(&buf[..n]).unwrap();
                        json
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // merge all chunks into one file
                if n.cmd == "merge" {
                    let file_manager = FileManagerService;
                    let merge_data: Result<MergeRequestData> = match n.data {
                        MicroRequestData::Merge(data) => Ok(data),
                        _ => Err(anyhow::anyhow!("data is not MergeRequestData")),
                    };
                    let merge_data = merge_data.unwrap();
                    file_manager
                        .merge(&merge_data.file_key, merge_data.total_chunk)
                        .await
                        .unwrap();

                // delete all chunks and the directory
                } else if n.cmd == "delete" {
                    let file_manager = services::file_manager_service::FileManagerService;
                    let delete_data: Result<DeleteRequestData> = match n.data {
                        MicroRequestData::Delete(data) => Ok(data),
                        _ => Err(anyhow::anyhow!("data is not DeleteRequestData")),
                    };
                    let delete_data = delete_data.unwrap();
                    file_manager.delete(&delete_data.file_key).await.unwrap();
                }

                if let Err(e) = socket.write_all(n.cmd.as_bytes()).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

async fn rabbit() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare("storage", QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = std::str::from_utf8(&delivery.body)?;
                println!("({:>3}) Received [{}]", i, body);
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    let _ = connection.close();
    Ok(())
}
