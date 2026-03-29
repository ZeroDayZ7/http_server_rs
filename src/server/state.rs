use crate::config::Settings;
use crate::domain::{UserRepository, VaultRepository};
use crate::errors::AppResult;

use crate::infrastructure::crypto::aes_service::AesCryptoService;
use crate::infrastructure::redis::RedisService;
use crate::infrastructure::redis_rate_limiter::RedisRateLimiter;
use crate::infrastructure::{MongoUserRepository, MongoVaultRepository};

use crate::services::user_service::UserService;
use crate::services::vault::vault_decoder::JsonVaultDecoder;
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
        let mongo_db = crate::infrastructure::database::init_mongo(&settings).await?;
        let db_pool = Arc::new(mongo_db);

        let vault_repo = Arc::new(MongoVaultRepository::new(Arc::clone(&db_pool)));
        let user_repo = Arc::new(MongoUserRepository::new(Arc::clone(&db_pool)));

        let crypto_service = Arc::new(AesCryptoService);
        let decoder = Arc::new(JsonVaultDecoder);

        let vault_service = Arc::new(VaultService::new(
            vault_repo as Arc<dyn VaultRepository>,
            crypto_service,
            decoder,
        ));

        let user_service = Arc::new(UserService::new(user_repo as Arc<dyn UserRepository>));

        let redis_service = Arc::new(RedisService::new(&settings).await?);
        let redis_rate_limiter = Arc::new(RedisRateLimiter::new(redis_service).await);

        Ok(Self {
            settings,
            vault_service,
            user_service,
            redis_rate_limiter,
        })
    }
}
