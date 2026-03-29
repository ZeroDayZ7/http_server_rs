use crate::config::Settings;
use crate::domain::vault::VaultRepository;
use crate::infrastructure::mongodb_vault::MongoVaultRepository;
use crate::infrastructure::redis::RedisService;
use crate::infrastructure::redis_rate_limiter::RedisRateLimiter;
use crate::repository::UserRepository;
use crate::repository::user_repo::MongoUserRepository;
use crate::services::user_service::UserService;
use crate::services::vault_service::VaultService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub vault_service: Arc<VaultService>,
    pub user_service: Arc<UserService>,
    pub redis_rate_limiter: Arc<RedisRateLimiter>,
}

impl AppState {
    pub async fn new(settings: Arc<Settings>) -> Self {
        // 1. Inicjalizacja bazy danych (Jeden punkt wejścia)
        let mongo_db = crate::infrastructure::database::init_mongo(&settings).await;
        let db_pool = Arc::new(mongo_db);

        // 2. Repozytoria (Wstrzykujemy ten sam db_pool do obu)
        // Używamy Arc::clone(&db_pool) zamiast db_pool.clone() dla jasności
        let vault_repo = Arc::new(MongoVaultRepository::new(Arc::clone(&db_pool)));
        let user_repo = Arc::new(MongoUserRepository::new(Arc::clone(&db_pool)));

        // 3. Serwisy (Abstrakcyjne - przyjmują traity)
        // Rzutujemy konkretne repozytoria na dyn Trait + Send + Sync
        let vault_service = Arc::new(VaultService::new(
            vault_repo as Arc<dyn VaultRepository + Send + Sync>,
        ));

        let user_service = Arc::new(UserService::new(
            user_repo as Arc<dyn UserRepository + Send + Sync>,
        ));

        // 4. Infrastruktura Redis
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
