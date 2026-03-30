use crate::config::Settings;
use crate::errors::{AppError, AppResult};
use fred::prelude::*;
use std::sync::Arc;
use std::time::Duration;

pub struct RedisManager {
    client: Client, // Pole jest prywatne
}

impl RedisManager {
    pub async fn new(settings: &Settings) -> AppResult<Arc<Self>> {
        let db_index: u8 = settings.redis.db.try_into().map_err(|_| {
            AppError::ValidationError("Redis DB index must be between 0-255".to_string())
        })?;

        let config = Config {
            server: ServerConfig::Centralized {
                server: Server::new(&settings.redis.host, settings.redis.port),
            },
            password: settings.redis.password.clone(),
            database: Some(db_index),
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
                Ok(Arc::new(Self { client }))
            }
            Ok(Err(e)) => Err(AppError::Internal(anyhow::anyhow!("Błąd Redisa: {}", e))),
            Err(_) => Err(AppError::Internal(anyhow::anyhow!(
                "Timeout połączenia z Redis"
            ))),
        }
    }

    pub async fn set_ex(&self, key: &str, value: &str, ttl_sec: u64) -> AppResult<()> {
        self.client
            .set::<(), _, _>(
                key,
                value,
                Some(Expiration::EX(ttl_sec as i64)),
                None,
                false,
            )
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis SET error: {}", e)))
    }

    pub async fn get(&self, key: &str) -> AppResult<Option<String>> {
        self.client
            .get(key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis GET error: {}", e)))
    }

    pub async fn del(&self, key: &str) -> AppResult<()> {
        self.client
            .del::<(), _>(key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis DEL error: {}", e)))
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }
}
