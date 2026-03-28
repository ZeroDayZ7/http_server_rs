use crate::config::Settings;

use fred::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub struct RedisService {
    client: Client,
}

impl RedisService {
    pub async fn new(settings: &Settings) -> Self {
        let config = fred::prelude::Config {
            server: ServerConfig::Centralized {
                server: Server::new(&settings.redis.host, settings.redis.port),
            },
            password: settings.redis.password.clone(),
            database: Some(
                settings
                    .redis
                    .db
                    .try_into()
                    .expect("Redis DB index must be between 0-255"),
            ),
            ..Default::default()
        };

        let client = Client::new(config, None, None, None);

        client.connect();

        match tokio::time::timeout(Duration::from_secs(5), client.wait_for_connect()).await {
            Ok(Ok(_)) => {
                tracing::info!(
                    "🚀 Połączono z Redis: {}:{}",
                    settings.redis.host,
                    settings.redis.port
                );
            }
            _ => {
                panic!(
                    "❌ Krytyczny błąd: Nie udało się połączyć z Redisem na {}:{}",
                    settings.redis.host, settings.redis.port
                );
            }
        }

        Self { client }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub async fn set_auth_token(
        &self,
        user_id: &str,
        token: &str,
        ttl_sec: u64,
    ) -> Result<(), fred::error::Error> {
        self.client
            .set(
                user_id,
                token,
                Some(Expiration::EX(ttl_sec as i64)),
                None,
                false,
            )
            .await
    }

    pub async fn get_auth_token(
        &self,
        user_id: &str,
    ) -> Result<Option<String>, fred::error::Error> {
        // Updated type
        self.client.get(user_id).await
    }
}
