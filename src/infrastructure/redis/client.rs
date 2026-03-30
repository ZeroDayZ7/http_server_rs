use crate::config::Settings;
use crate::errors::{AppError, AppResult};
use fred::prelude::*;
use std::sync::Arc;
use std::time::Duration;

pub struct RedisManager {
    client: Client,
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
            Ok(Err(e)) => Err(Self::redis_error(e)),
            Err(_) => Err(AppError::Internal(anyhow::anyhow!(
                "Timeout połączenia z Redis"
            ))),
        }
    }

    pub async fn set_ex(&self, key: &str, value: &str, ttl: u64) -> AppResult<()> {
        self.client
            .set::<(), _, _>(key, value, Some(Expiration::EX(ttl as i64)), None, false)
            .await
            .map_err(Self::redis_error)
    }

    pub async fn get(&self, key: &str) -> AppResult<Option<String>> {
        self.client.get(key).await.map_err(Self::redis_error)
    }

    pub async fn del(&self, key: &str) -> AppResult<()> {
        self.client
            .del::<(), _>(key)
            .await
            .map_err(Self::redis_error)
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    fn redis_error<E: std::fmt::Display>(e: E) -> AppError {
        AppError::Internal(anyhow::anyhow!("Redis error: {}", e))
    }
}
