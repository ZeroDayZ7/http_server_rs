use crate::domain::UserRepository;
use crate::domain::user::User;
use crate::errors::AppResult;
use async_trait::async_trait;
use mongodb::{Database, bson::doc};
use std::sync::Arc;

pub struct MongoUserRepository {
    db: Arc<Database>,
}

impl MongoUserRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let collection = self.db.collection::<User>("users");
        let user = collection.find_one(doc! { "email": email }).await?;
        Ok(user)
    }

    async fn save(&self, user: User) -> AppResult<()> {
        let collection = self.db.collection::<User>("users");
        collection.insert_one(user).await?;
        Ok(())
    }
}
