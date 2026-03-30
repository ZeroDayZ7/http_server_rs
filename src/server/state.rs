use crate::config::Settings;
use crate::domain::ports::services::{UserServicePort, VaultServicePort};
use crate::errors::AppResult;
use crate::infrastructure::crypto::aes_service::AesCryptoService;
use crate::infrastructure::redis::client::RedisManager;
use crate::infrastructure::redis::rate_limiter::RedisRateLimiter;
use crate::infrastructure::serialization::JsonDecoder;
use crate::infrastructure::{MongoUserRepository, MongoVaultRepository};
use crate::services::user_service::UserService;
use crate::services::vault::vault_service::VaultService;
use mongodb::Database;
use std::sync::Arc;

/// Grupuje logikę biznesową ukrytą za portami (abstrakcja)
pub struct Services {
    pub vault: Arc<dyn VaultServicePort>,
    pub user: Arc<dyn UserServicePort>,
}

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    /// Wszystkie serwisy domenowe pod jednym kluczem
    pub services: Arc<Services>,
    /// Infrastruktura wymagana przez middleware/healthcheck
    pub redis_rate_limiter: Arc<RedisRateLimiter>,
    pub db: Database,
    pub redis_manager: Arc<RedisManager>,
}

impl AppState {
    pub async fn new(settings: Arc<Settings>) -> AppResult<Self> {
        // 1. Warstwa Infrastruktury (Połączenia)
        let mongo_db = crate::infrastructure::database::init_mongo(&settings.database).await?;
        let redis_manager = Arc::new(RedisManager::new(&settings.redis).await?);

        let db_pool = Arc::new(mongo_db.clone());

        // 2. Warstwa Danych (Repozytoria)
        let vault_repo = Arc::new(MongoVaultRepository::new(Arc::clone(&db_pool)));
        let user_repo = Arc::new(MongoUserRepository::new(Arc::clone(&db_pool)));

        // 3. Komponenty pomocnicze
        let crypto_service = Arc::new(AesCryptoService::new(settings.crypto.clone()));
        let decoder = Arc::new(JsonDecoder);

        // 4. Inicjalizacja konkretnych serwisów
        let vault_service_impl = Arc::new(VaultService::new(
            vault_repo,
            crypto_service,
            decoder,
            settings.crypto.clone(),
        ));

        let user_service_impl = Arc::new(UserService::new(user_repo));

        // 5. Budowa stanu z rzutowaniem na Porty (dyn Trait)
        Ok(Self {
            settings,
            services: Arc::new(Services {
                vault: vault_service_impl as Arc<dyn VaultServicePort>,
                user: user_service_impl as Arc<dyn UserServicePort>,
            }),
            redis_rate_limiter: Arc::new(RedisRateLimiter::new(Arc::clone(&redis_manager)).await),
            db: mongo_db,
            redis_manager,
        })
    }
}
