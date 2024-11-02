use anyhow::{bail, Result};
use tower_sessions::Session;

pub enum ValidMethod {
    Write,
    Read,
}

pub trait SessionManagerService {
    async fn is_available_key(
        &self,
        session: &Session,
        file_key: &String,
        method: ValidMethod,
    ) -> Result<bool>;
    async fn reset(&self, session: &Session, file_key: String) -> Result<()>;
}

pub struct SessionManager;

impl SessionManagerService for SessionManager {
    async fn is_available_key(
        &self,
        session: &Session,
        file_key: &String,
        method: ValidMethod,
    ) -> Result<bool> {
        let key = match method {
            ValidMethod::Write => format!("write:{}", file_key),
            ValidMethod::Read => format!("read:{}", file_key),
        };
        let is_available = session.get::<bool>(&key).await?;

        match is_available {
            Some(is_available) => Ok(is_available),
            None => bail!("file_key not found"),
        }
    }
    async fn reset(&self, session: &Session, file_key: String) -> Result<()> {
        session.remove::<String>(&file_key).await?;
        Ok(())
    }
}
