use anyhow::Result;
use axum::{body::Body, extract::{Multipart, Path}, response::Response};
use tower_sessions::Session;

use crate::{
    routes::{AppError, BodyBuilder, ResponseBody},
    services::{file_key_service::FileKey, FileKeyService},
};

pub async fn write(
    Path(file_key): Path<String>,
    session: Session, mut multipart: Multipart) -> Result<Response<Body>, AppError> {
    let is_available = FileKeyService.is_available_key(&session, file_key).await?;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let key = field.chunk
        let data = field.bytes().await?;
    }

    let body = ResponseBody::new("file write success".to_string()).build_body();

    let response = Response::builder().status(200).body(body).unwrap();

    Ok(response)
}
