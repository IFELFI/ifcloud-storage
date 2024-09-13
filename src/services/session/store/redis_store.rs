use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client, RedisResult};

use super::Store;

#[derive(Clone)]
pub struct RedisStore {
    client: redis::Client,
}

#[allow(dead_code)]
impl RedisStore {
    pub fn new(addr: &str) -> RedisStore {
        let client = Client::open(addr).unwrap();
        RedisStore { client }
    }

    async fn connection(&self) -> RedisResult<MultiplexedConnection> {
        self.client.get_multiplexed_tokio_connection().await
    }
}

#[async_trait]
impl Store for RedisStore {
    async fn save(&self, key: String, value: String) -> Result<(), String> {
        let mut con: MultiplexedConnection = match __self.connection().await {
            Ok(con) => con,
            _ => return Err("Failed to connect to Redis".to_string()),
        };
        let result: RedisResult<()> = con.set(key, value).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to save to Redis".to_string()),
        }
    }

    async fn get(&self, key: String) -> Result<Option<String>, String> {
        let mut con: MultiplexedConnection = match __self.connection().await {
            Ok(con) => con,
            _ => return Err("Failed to connect to Redis".to_string()),
        };
        let result: RedisResult<String> = con.get(key).await;
        match result {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }

    async fn delete(&self, key: String) -> Result<(), String> {
        let mut con: MultiplexedConnection = match __self.connection().await {
            Ok(con) => con,
            _ => return Err("Failed to connect to Redis".to_string()),
        };
        let result: RedisResult<()> = con.del(key).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to delete from Redis".to_string()),
        }
    }
}
