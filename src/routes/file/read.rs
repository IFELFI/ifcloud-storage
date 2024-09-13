use crate::services::auth_service::{AuthService, Authentication};
use crate::services::session::store::redis_store::RedisStore;

pub async fn read(key: String) -> String {
    let store = RedisStore::new("redis://127.0.0.1:6379");
    let session = AuthService::new("auth".to_string(), store);
    let result = session.verify_session(&key).await;
    match result {
        Ok(value) => {
            if value {
                "Session is valid".to_string()
            } else {
                "Session is invalid".to_string()
            }
        }
        Err(value) => value,
    }
}
