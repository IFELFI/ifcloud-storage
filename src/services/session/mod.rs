use async_trait::async_trait;

#[derive(Clone)]
pub struct Session<S> {
    store: S,
    prefix: String,
}

#[allow(dead_code)]
impl<S> Session<S>
where
    S: SessionStore,
{
    pub fn new(store: S, prefix: Option<String>) -> Session<S> {
        let prefix = prefix.unwrap_or("".to_string());
        Session { store, prefix }
    }

    pub async fn save(&self, key: String, value: String) -> Result<(), String> {
        let formatted_key = format!("{}{}", self.prefix, key);
        self.store.save(formatted_key, value).await
    }

    pub async fn get(&self, key: String) -> Result<Option<String>, String> {
        let formatted_key = format!("{}{}", self.prefix, key);
        self.store.get(formatted_key).await
    }

    pub async fn delete(&self, key: String) -> Result<(), String> {
        let formatted_key = format!("{}{}", self.prefix, key);
        self.store.delete(formatted_key).await
    }
}

#[async_trait]
pub trait SessionStore {
    async fn save(&self, key: String, value: String) -> Result<(), String>;
    async fn get(&self, key: String) -> Result<Option<String>, String>;
    async fn delete(&self, key: String) -> Result<(), String>;
}
