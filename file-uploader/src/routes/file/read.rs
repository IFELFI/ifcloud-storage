use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::Response,
};
use serde::Deserialize;
use tokio_util::io::ReaderStream;

use crate::routes::{AppError, BodyBuilder, ResponseBody};

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(default)]
    file_name: Option<String>,
}

pub async fn read(
    Path(file_key): Path<String>,
    Query(params): Query<Params>,
) -> Result<Response<Body>, AppError> {
    // Check file_name is given
    let file_name = match params.file_name {
        Some(file_name) => file_name,
        None => {
            let body = ResponseBody::new("file_name is required".to_string()).build_body();

            let response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
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
    let content_type = get_content_type(&file_name);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", file_name),
        )
        .header("Cache-Control", "no-cache")
        .header("Pragma", "no-cache")
        .body(body)
        .unwrap();

    Ok(response)
}

fn get_content_type(file_name: &str) -> &str {
    let ext = file_name.split('.').last().unwrap_or("none");

    let file_type = match ext {
        // Image
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        // Application
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "7z" => "application/x-7z-compressed",
        "rar" => "application/vnd.rar",
        "json" => "application/json",
        "xml" => "application/xml",
        "csv" => "text/csv",
        // Video
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "ogg" => "video/ogg",
        // Audio
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "flac" => "audio/flac",
        // Text
        "txt" => "text/plain",
        "md" => "text/markdown",
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        // If none of them match, return "application/octet-stream"
        _ => "application/octet-stream",
    };

    file_type
}
