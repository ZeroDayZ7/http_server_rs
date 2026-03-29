use crate::config::Settings;
use crate::errors::{AppError, AppResult};
use fred::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub struct RedisService {
    client: Client,
}

impl RedisService {
    pub async fn new(settings: &Settings) -> AppResult<Self> {
        let db_index: u8 = settings.redis.db.try_into().map_err(|_| {
            AppError::ValidationError("Redis DB index must be between 0-255".to_string())
        })?;

        let config = fred::prelude::Config {
            server: ServerConfig::Centralized {
                server: Server::new(&settings.redis.host, settings.redis.port),
            },
            password: settings.redis.password.clone(),
            database: Some(db_index),
            ..Default::default()
        };

        let client = Client::new(config, None, None, None);

        // Rozpoczynamy połączenie w tle
        client.connect();

        // Czekamy na połączenie z timeoutem (zamiast panic!)
        match tokio::time::timeout(Duration::from_secs(5), client.wait_for_connect()).await {
            Ok(Ok(_)) => {
                tracing::info!(
                    "🚀 Połączono z Redis: {}:{}",
                    settings.redis.host,
                    settings.redis.port
                );
            }
            Ok(Err(e)) => {
                return Err(AppError::Internal(anyhow::anyhow!(
                    "Błąd połączenia z Redis: {}",
                    e
                )));
            }
            Err(_) => {
                return Err(AppError::Internal(anyhow::anyhow!(
                    "Timeout: Nie udało się połączyć z Redisem na {}:{}",
                    settings.redis.host,
                    settings.redis.port
                )));
            }
        }

        Ok(Self { client })
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
        self.client.get(user_id).await
    }
}
