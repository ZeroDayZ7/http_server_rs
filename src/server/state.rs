use crate::config::Settings;
use crate::errors::AppResult;
use crate::infrastructure::crypto::aes_service::AesCryptoService;
use crate::infrastructure::redis::client::RedisManager;
use crate::infrastructure::redis::rate_limiter::RedisRateLimiter;
use crate::infrastructure::serialization::JsonDecoder;
use crate::infrastructure::{MongoUserRepository, MongoVaultRepository};
use crate::services::user_service::UserService;
use crate::services::vault::vault_service::VaultService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub vault_service: Arc<VaultService>,
    pub user_service: Arc<UserService>,
    pub redis_rate_limiter: Arc<RedisRateLimiter>,
}

impl AppState {
    pub async fn new(settings: Arc<Settings>) -> AppResult<Self> {
        // 1. DB: Przekazujemy tylko fragment .database
        let mongo_db = crate::infrastructure::database::init_mongo(&settings.database).await?;
        let db_pool = Arc::new(mongo_db);

        // 2. Repozytoria
        let vault_repo = Arc::new(MongoVaultRepository::new(Arc::clone(&db_pool)));
        let user_repo = Arc::new(MongoUserRepository::new(Arc::clone(&db_pool)));

        // 3. Infrastruktura i Serwisy pomocnicze
        let crypto_service = Arc::new(AesCryptoService::new(settings.crypto.clone()));
        let decoder = Arc::new(JsonDecoder);

        // 4. Redis: Przekazujemy tylko fragment .redis (zakładając zmianę w RedisManager)
        let redis_manager = RedisManager::new(&settings.redis).await?;
        let redis_rate_limiter = Arc::new(RedisRateLimiter::new(Arc::new(redis_manager)).await);

        let vault_service = Arc::new(VaultService::new(
            vault_repo,
            crypto_service,
            decoder,
            settings.crypto.clone(),
        ));

        let user_service = Arc::new(UserService::new(user_repo));

        Ok(Self {
            settings,
            vault_service,
            user_service,
            redis_rate_limiter,
        })
    }
}
