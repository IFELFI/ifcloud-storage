use std::time::Duration;

use async_trait::async_trait;
use redis::{Client, RedisResult};

#[allow(dead_code)]
pub struct TokenService {
    client: Client,
    connection: redis::aio::MultiplexedConnection,
}

#[async_trait]
#[allow(dead_code)]
pub trait Token {
    async fn get_file_key(&self, token: &str) -> RedisResult<String>;
}

#[allow(dead_code)]
impl TokenService {
    pub async fn new(url: &str) -> RedisResult<Self> {
        let client = Client::open(url)?;
        let connection = client
            .get_multiplexed_tokio_connection_with_response_timeouts(
                Duration::from_secs(10),
                Duration::from_secs(10),
            )
            .await?;
        Ok(Self { client, connection })
    }
}

#[async_trait]
impl Token for TokenService {
    async fn get_file_key(&self, token: &str) -> RedisResult<String> {
        let mut con = self.connection.clone();
        let value: String = redis::cmd("GET").arg(&token).query_async(&mut con).await?;
        let _: () = redis::cmd("DEL").arg(&token).query_async(&mut con).await?;
        Ok(value)
    }
}
