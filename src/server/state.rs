use std::sync::Arc;

use crate::config::Settings;
// Importujemy traity z domeny
use crate::domain::{UserRepository, VaultRepository};
// Importujemy konkretne implementacje z infrastruktury (używając Twoich re-eksportów)
use crate::infrastructure::redis::RedisService;
use crate::infrastructure::redis_rate_limiter::RedisRateLimiter;
use crate::infrastructure::{MongoUserRepository, MongoVaultRepository};
use crate::services::user_service::UserService;
use crate::services::vault_service::VaultService;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub vault_service: Arc<VaultService>,
    pub user_service: Arc<UserService>,
    pub redis_rate_limiter: Arc<RedisRateLimiter>,
}

impl AppState {
    pub async fn new(settings: Arc<Settings>) -> Self {
        // 1. DB
        let mongo_db = crate::infrastructure::database::init_mongo(&settings).await;
        let db_pool = Arc::new(mongo_db);

        // 2. Repozytoria
        let vault_repo = Arc::new(MongoVaultRepository::new(Arc::clone(&db_pool)));
        let user_repo = Arc::new(MongoUserRepository::new(Arc::clone(&db_pool)));

        // 3. Serwisy
        let vault_service = Arc::new(VaultService::new(vault_repo as Arc<dyn VaultRepository>));

        let user_service = Arc::new(UserService::new(user_repo as Arc<dyn UserRepository>));

        // 4. Redis
        let redis_service = Arc::new(RedisService::new(&settings).await);
        let redis_rate_limiter = Arc::new(RedisRateLimiter::new(redis_service).await);

        Self {
            settings,
            vault_service,
            user_service,
            redis_rate_limiter,
        }
    }
}
