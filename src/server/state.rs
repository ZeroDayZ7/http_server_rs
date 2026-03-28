use crate::config::Settings;
use crate::infrastructure::mongodb_vault::MongoVaultRepository;
use crate::infrastructure::redis::RedisService;
use crate::infrastructure::redis_rate_limiter::RedisRateLimiter;

use crate::repository::user_repo::UserRepository;
use mongodb::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub redis_rate_limiter: RedisRateLimiter,
    pub db: Database,
    pub user_repo: Arc<UserRepository>,
    pub db_repo: Arc<MongoVaultRepository>,
}

impl AppState {
    pub async fn new(settings: Arc<Settings>) -> Self {
        // 1. Init Infrastructure
        let redis_service = RedisService::new(&settings).await;
        let redis_arc = Arc::new(redis_service);

        let mongo_db = crate::infrastructure::database::init_mongo(&settings).await;
        let mongo_arc = Arc::new(mongo_db.clone());

        // 2. Init Rate Limiter
        let redis_rate_limiter = RedisRateLimiter::new(redis_arc).await;

        // 3. Init Repositories
        let user_repo = Arc::new(UserRepository::new(Arc::new(mongo_db.clone())));
        // let db_repo = Arc::new(MongoVaultRepository::new(mongo_arc.clone()));

        // let db_repo: Arc<dyn VaultRepository> =
        //     Arc::new(MongoVaultRepository::new(mongo_arc.clone()));

        // let db_repo =
        //     Arc::new(MongoVaultRepository::new(mongo_arc.clone())) as Arc<dyn VaultRepository>;

        // let db_repo: Arc<dyn VaultRepository> =
        //     Arc::new(MongoVaultRepository::new(mongo_arc.clone()));

        // let repo = MongoVaultRepository::new(mongo_arc.clone());
        // let db_repo = Arc::new(repo) as Arc<dyn VaultRepository>;

        let repo = MongoVaultRepository::new(mongo_arc.clone());
        let db_repo = Arc::new(repo);
        Self {
            settings,
            redis_rate_limiter,
            db: mongo_db,
            user_repo,
            db_repo,
        }
    }
}
