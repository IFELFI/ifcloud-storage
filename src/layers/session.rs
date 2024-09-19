use anyhow::Result;
use tower_sessions::{
    cookie::{time::Duration, Key, SameSite},
    service::PrivateCookie,
    Expiry, SessionManagerLayer,
};
use tower_sessions_redis_store::{
    fred::prelude::{ClientLike, RedisClient},
    RedisStore,
};

pub async fn set_session_layer(
) -> Result<SessionManagerLayer<RedisStore<RedisClient>, PrivateCookie>> {
    let client = RedisClient::default();
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
