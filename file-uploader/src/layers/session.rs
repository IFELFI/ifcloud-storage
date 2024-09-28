use std::env;

use anyhow::Result;
use tower_sessions::{
    cookie::{time::Duration, Key},
    service::PrivateCookie,
    Expiry, SessionManagerLayer,
};
use tower_sessions_redis_store::{
    fred::{prelude::{ClientLike, RedisClient}, types::RedisConfig},
    RedisStore,
};

pub async fn set_session_layer(
) -> Result<SessionManagerLayer<RedisStore<RedisClient>, PrivateCookie>> {
    let addr = env::var("SESSION_STORAGE_URL").unwrap_or("redis://127.0.0.1:6379".to_string());
    let client = RedisClient::new(RedisConfig::from_url(&addr)?, None, None, None);
    client.init().await?;

    let cookie_secret = Key::generate();

    let session_store = RedisStore::new(client);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_private(cookie_secret)
        //.with_secure(true)
        //.with_same_site(SameSite::Lax)
        .with_always_save(true)
        //.with_domain("ifelfi.com")
        .with_path("/")
        .with_name("storage_session")
        .with_expiry(Expiry::OnInactivity(Duration::seconds(60)));

    Ok(session_layer)
}
