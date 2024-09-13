use super::{
    session::{store::redis_store::RedisStore, Session},
    Service,
};
use async_trait::async_trait;

#[async_trait]
#[allow(dead_code)]
pub trait Authentication {
    async fn save_session(&self, session: &str) -> Result<(), String>;
    async fn verify_session(&self, session: &str) -> Result<bool, String>;
}

#[derive(Clone)]
pub struct AuthService {
    name: String,
    session: Session<RedisStore>,
}

#[allow(dead_code)]
impl AuthService {
    pub fn new(name: String, store: RedisStore) -> AuthService {
        AuthService {
            name,
            session: Session::new(store, None),
        }
    }

    pub fn with_prefix(name: String, store: RedisStore, prefix: String) -> AuthService {
        AuthService {
            name,
            session: Session::new(store, Some(prefix)),
        }
    }
}

impl Service for AuthService {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[async_trait]
impl Authentication for AuthService {
    async fn save_session(&self, session: &str) -> Result<(), String> {
        self.session.save(session.to_string(), "".to_string()).await
    }
    async fn verify_session(&self, session: &str) -> Result<bool, String> {
        let result = self.session.get(session.to_string()).await;
        match result {
            Ok(value) => match value {
                Some(_) => Ok(true),
                None => Ok(false),
            },
            Err(value) => Err(value),
        }   
    }
}
