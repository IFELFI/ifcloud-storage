use anyhow::Result;
use async_trait::async_trait;
use axum::{body::Body, extract::Path, response::Response};
use tower_sessions::Session;

use crate::services::{token_service::Token, TokenService};

use super::{AppError, BodyBuilder, ResponseBody};

#[async_trait]
pub trait IssueSession {
    async fn new_session(
        &self,
        Path(token): Path<String>,
        session: &Session,
    ) -> Result<Response<Body>, AppError>;
}

pub struct SessionIssueRoute {
    token_service: TokenService,
}

impl SessionIssueRoute {
    pub fn new(token_service: TokenService) -> Self {
        Self { token_service }
    }
}

#[async_trait]
impl IssueSession for SessionIssueRoute {
    async fn new_session(
        &self,
        Path(token): Path<String>,
        session: &Session,
    ) -> Result<Response<Body>, AppError> {
        let file_key = self.token_service.get_file_key(&token).await?;
        session.insert(&file_key, true).await?;

        let body = ResponseBody::new("session issued".to_string()).build_body();

        let response = Response::builder().status(200).body(body).unwrap();

        Ok(response)
    }
}

pub async fn issue_session(
    Path(token): Path<String>,
    session: Session,
) -> Result<Response<Body>, AppError> {
    let redis_url = std::env::var("TOKEN_STORAGE_URL").unwrap_or("redis://127.0.0.1:6666".to_string());
    let token_service = TokenService::new(&redis_url).await?;
    let route = SessionIssueRoute::new(token_service);

    route.new_session(Path(token), &session).await
}
