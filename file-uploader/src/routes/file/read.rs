use anyhow::Result;
use axum::{body::Body, extract::Path, http::StatusCode, response::Response};
use tokio_util::io::ReaderStream;
use tower_sessions::Session;

use crate::{
    routes::{AppError, BodyBuilder, ResponseBody},
    services::{
        session_manager::{SessionManagerService, ValidMethod},
        SessionManager,
    },
};

pub async fn read(
    Path(file_key): Path<String>,
    session: Session,
) -> Result<Response<Body>, AppError> {
    match SessionManager
        .is_available_key(&session, &file_key, ValidMethod::Read)
        .await
    {
        Ok(is_available) => {
            if !is_available {
                let body = ResponseBody::new("file_key is not available".to_string()).build_body();

                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(body)
                    .unwrap();

                return Ok(response);
            }
        }
        Err(err) => {
            let body = ResponseBody::new(format!("session error: {}", err)).build_body();

            let response = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body)
                .unwrap();

            return Ok(response);
        }
    };

    let base_storage_path = std::env::var("BASE_STORAGE_PATH").unwrap_or("./storage".to_string());
    let original_file_name = "original";
    let file_path = format!("{}/{}/{}", base_storage_path, file_key, original_file_name);

    let file = match tokio::fs::File::open(file_path).await {
        Ok(file) => file,
        Err(err) => {
            let body = ResponseBody::new(format!("file not found: {}", err)).build_body();

            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body)
                .unwrap();

            return Ok(response);
        }
    };
    let stream = ReaderStream::new(file);

    let body = Body::from_stream(stream);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/octet-stream")
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", file_key),
        )
        .header("Cache-Control", "no-cache")
        .header("Pragma", "no-cache")
        .body(body)
        .unwrap();

    Ok(response)
}
