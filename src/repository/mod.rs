use crate::domain::user::User;
use crate::errors::AppResult;
use async_trait::async_trait;

// Definiujemy interfejs jako publiczny
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn save(&self, user: User) -> AppResult<()>;
}

// Deklarujemy moduł z implementacją
pub mod user_repo;
