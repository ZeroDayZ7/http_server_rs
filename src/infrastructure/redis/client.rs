use crate::config::Settings as AppSettings;
use crate::errors::{AppError, AppResult};
use fred::prelude::*;
use std::time::Duration;

pub struct RedisManager {
    client: Client,
}

impl RedisManager {
    pub async fn new(settings: &AppSettings) -> AppResult<Self> {
        let db_index: u8 = settings.redis.db.try_into().map_err(|_| {
            AppError::ValidationError(
                "Index bazy danych Redis musi mieścić się w zakresie 0-255".into(),
            )
        })?;

        let reconnect_policy = ReconnectPolicy::new_exponential(0, 100, 5000, 2);

        let config = Config {
            server: ServerConfig::Centralized {
                server: Server::new(&settings.redis.host, settings.redis.port),
            },
            password: settings.redis.password.clone(),
            database: Some(db_index),
            ..Default::default()
        };

        let perf_config = PerformanceConfig::default();

        let client = Client::new(config, Some(perf_config), None, Some(reconnect_policy));

        client.connect();

        match tokio::time::timeout(Duration::from_secs(5), client.wait_for_connect()).await {
            Ok(Ok(_)) => {
                tracing::info!(
                    host = %settings.redis.host,
                    port = %settings.redis.port,
                    "🚀 Połączono z Redis"
                );
                Ok(Self { client })
            }
            Ok(Err(e)) => Err(AppError::from(e)),
            Err(_) => Err(AppError::Internal(anyhow::anyhow!(
                "Timeout podczas inicjalizacji połączenia z Redis"
            ))),
        }
    }

    pub async fn set_ex(&self, key: &str, value: &str, ttl_sec: u64) -> AppResult<()> {
        let expiration = Expiration::EX(
            ttl_sec
                .try_into()
                .map_err(|_| AppError::ValidationError("TTL jest zbyt duży".into()))?,
        );

        self.client
            .set::<(), _, _>(key, value, Some(expiration), None, false)
            .await
            .map_err(AppError::from)
    }

    pub async fn get(&self, key: &str) -> AppResult<Option<String>> {
        self.client
            .get::<Option<String>, _>(key)
            .await
            .map_err(AppError::from)
    }

    pub async fn del(&self, key: &str) -> AppResult<()> {
        self.client.del::<(), _>(key).await.map_err(AppError::from)
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
