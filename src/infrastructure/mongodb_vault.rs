use crate::domain::VaultRepository;
use crate::domain::vault::EncryptedCV;
use crate::errors::AppResult;
use async_trait::async_trait;
use mongodb::{Database, bson::doc};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

pub struct MongoVaultRepository {
    db: Arc<Database>,
    collection_name: String,
}

impl MongoVaultRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            collection_name: "vaults".to_string(),
        }
    }
}

#[async_trait]
impl VaultRepository for MongoVaultRepository {
    async fn get_cv_by_id(&self, id: &str) -> AppResult<Option<EncryptedCV>> {
        let collection = self.db.collection::<EncryptedCV>(&self.collection_name);
        let filter = doc! { "id": id };

        let mut attempts = 0;

        loop {
            attempts += 1;

            let result = timeout(Duration::from_secs(3), collection.find_one(filter.clone())).await;

            match result {
                Ok(inner) => return Ok(inner?),
                Err(_) if attempts < 2 => continue,
                Err(e) => return Err(e.into()),
            }
        }
    }
}
