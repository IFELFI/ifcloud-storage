use std::time::Duration;

use redis::{Client, RedisResult};

pub struct TokenManager {
    //client: Client,
    connection: redis::aio::MultiplexedConnection,
}

pub trait TokenManagerService {
    async fn get_file_key(&self, token: &str) -> RedisResult<String>;
}

impl TokenManager {
    pub async fn new(url: &str) -> RedisResult<Self> {
        let client = Client::open(url)?;
        let connection = client
            .get_multiplexed_tokio_connection_with_response_timeouts(
                Duration::from_secs(10),
                Duration::from_secs(10),
            )
            .await?;
        Ok(Self { connection })
    }
}

impl TokenManagerService for TokenManager {
    async fn get_file_key(&self, token: &str) -> RedisResult<String> {
        let mut con = self.connection.clone();
        let value: String = redis::cmd("GET").arg(&token).query_async(&mut con).await?;
        let _: () = redis::cmd("DEL").arg(&token).query_async(&mut con).await?;
        Ok(value)
    }
}
