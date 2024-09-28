use anyhow::Result;
use axum::{body::Body, extract::Path, response::Response};
use tower_sessions::Session;

use crate::{
    routes::{AppError, BodyBuilder, ResponseBody},
    services::{session_manager::SessionManagerService, SessionManager},
};

pub async fn read(
    Path(file_key): Path<String>,
    session: Session,
) -> Result<Response<Body>, AppError> {
    let is_available = SessionManager.is_available_key(&session, &file_key).await?;

    let body = ResponseBody::new(format!("file_key: {}", is_available)).build_body();

    let response = Response::builder().status(200).body(body).unwrap();

    Ok(response)
}
