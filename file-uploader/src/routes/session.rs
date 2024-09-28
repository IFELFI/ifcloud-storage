use anyhow::Result;
use async_trait::async_trait;
use axum::{body::Body, extract::Path, response::Response};
use tower_sessions::Session;

use crate::services::{
    session_manager::SessionManagerService, token_manager::TokenManagerService, SessionManager,
    TokenManager,
};

use super::{AppError, BodyBuilder, ResponseBody};

// traits
#[async_trait]
pub trait SessionManageService {
    async fn new_session(
        &self,
        Path(token): Path<String>,
        session: &Session,
    ) -> Result<Response<Body>, AppError>;
    async fn delete_session(
        &self,
        Path(token): Path<String>,
        session: &Session,
    ) -> Result<Response<Body>, AppError>;
}

// route
pub struct SessionRoute {
    session_manager: SessionManager,
    token_manager: TokenManager,
}

// impl traits
impl SessionRoute {
    pub fn new(session_manager: SessionManager, token_manager: TokenManager) -> Self {
        Self {
            session_manager,
            token_manager
        }
    }
}

#[async_trait]
impl SessionManageService for SessionRoute {
    async fn new_session(
        &self,
        Path(token): Path<String>,
        session: &Session,
    ) -> Result<Response<Body>, AppError> {
        let file_key = self.token_manager.get_file_key(&token).await?;
        session.insert(&file_key, true).await?;

        let body = ResponseBody::new("session issued".to_string()).build_body();

        let response = Response::builder().status(200).body(body).unwrap();

        Ok(response)
    }

    async fn delete_session(
        &self,
        Path(token): Path<String>,
        session: &Session,
    ) -> Result<Response<Body>, AppError> {
        let file_key = self.token_manager.get_file_key(&token).await?;
        self.session_manager.reset(&session, file_key).await?;


        let body = ResponseBody::new("deleted file key from session".to_string()).build_body();

        let response = Response::builder().status(200).body(body).unwrap();

        Ok(response)
    }
}

pub async fn issue_session(
    Path(token): Path<String>,
    session: Session,
) -> Result<Response<Body>, AppError> {
    let redis_url =
        std::env::var("TOKEN_STORAGE_URL").unwrap_or("redis://127.0.0.1:6666".to_string());
    let session_manager = SessionManager {};
    let token_service = TokenManager::new(&redis_url).await?;
    let route = SessionRoute::new(session_manager, token_service);

    route.new_session(Path(token), &session).await
}
