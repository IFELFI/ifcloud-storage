use std::env;

use anyhow::Result;
use axum::{
    body::{Body, Bytes},
    extract::{Multipart, Path},
    response::Response,
};
use tower_sessions::Session;

use crate::{
    routes::{AppError, BodyBuilder, ResponseBody},
    services::{session_manager::SessionManagerService, SessionManager},
};

pub async fn write(
    Path(file_key): Path<String>,
    session: Session,
    mut multipart: Multipart,
) -> Result<Response<Body>, AppError> {
    // Check file_key is available
    let is_available = SessionManager.is_available_key(&session, &file_key).await?;

    // Check file_key is available
    if !is_available {
        let body = ResponseBody::new("file_key is not available".to_string()).build_body();

        let response = Response::builder().status(400).body(body).unwrap();

        return Ok(response);
    }

    // Load chunkCount and fileData from multipart
    let mut chunk_count: u32 = 0;
    let mut file_data: Bytes = Bytes::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        match name.as_str() {
            "chunkCount" => {
                chunk_count = field.text().await?.parse::<u32>()?;
            }
            "fileData" => {
                file_data = field.bytes().await?;
            }
            _ => {}
        }
    }

    // Create folder if not exist
    let storage_path = env::var("BASE_STORAGE_PATH").unwrap_or("./storage".to_string());
    // Chunk folder path
    let dir_path = format!("{}/chunks/{}", &storage_path, &file_key);
    // Chunk file path
    let file_path = format!("{}/{}", &dir_path, chunk_count);

    // Write chunk file
    tokio::fs::create_dir_all(&dir_path).await?;
    tokio::fs::write(file_path, file_data).await?;

    let body = ResponseBody::new("chunk upload success".to_string()).build_body();

    let response = Response::builder().status(200).body(body).unwrap();

    Ok(response)
}
