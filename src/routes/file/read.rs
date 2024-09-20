use anyhow::Result;
use axum::{body::Body, extract::Path, response::Response};
use tower_sessions::Session;

use crate::{
    routes::{AppError, BodyBuilder, ResponseBody},
    services::{file_key_service::FileKey, FileKeyService},
};

pub async fn read(
    Path(file_key): Path<String>,
    session: Session,
) -> Result<Response<Body>, AppError> {
    let is_available = FileKeyService.is_available_key(&session, &file_key).await?;

    let body = ResponseBody::new(format!("file_key: {}", is_available)).build_body();

    let response = Response::builder().status(200).body(body).unwrap();

    Ok(response)
}
