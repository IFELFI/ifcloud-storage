use async_trait::async_trait;

pub mod redis_store;

#[async_trait]
#[allow(dead_code)]
pub trait Store {
    async fn save(&self, key: String, value: String) -> Result<(), String>;
    async fn get(&self, key: String) -> Result<Option<String>, String>;
    async fn delete(&self, key: String) -> Result<(), String>;
}
