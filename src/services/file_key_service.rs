use anyhow::{bail, Result};
use async_trait::async_trait;
use tower_sessions::Session;

#[async_trait]
#[allow(dead_code)]
pub trait FileKey {
    async fn is_available_key(&self, session: &Session, file_key: String) -> Result<bool>;
    async fn reset(&self, session: &Session, file_key: String) -> Result<()>;
}

pub struct FileKeyService;

#[async_trait]
impl FileKey for FileKeyService {
    async fn is_available_key(&self, session: &Session, file_key: String) -> Result<bool> {
        let is_available = session.get::<bool>(&file_key).await?;

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
