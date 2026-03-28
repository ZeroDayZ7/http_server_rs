use crate::config::Settings;
use fred::prelude::*;
use std::time::Duration;

pub async fn io_redis_client(settings: &Settings) -> Client {
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

    // 3. Rozpoczęcie połączenia
    client.connect();

    // 4. Timeout na start, żeby nie blokować maina w nieskończoność
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

    client
}
