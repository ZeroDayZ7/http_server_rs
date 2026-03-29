



use crate::domain::UserRepository;
use crate::domain::user::User;
use crate::errors::{AppError, AppResult};
use std::sync::Arc;
use tracing::instrument;

pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    #[instrument(skip(self), fields(user_email = %email))]
    pub async fn get_user_by_email(&self, email: &str) -> AppResult<User> {
        self.repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Użytkownik {} nie istnieje", email)))
    }

    #[instrument(skip(self, user), fields(user_id = ?user.id))]
    pub async fn register_user(&self, user: User) -> AppResult<()> {
        self.repo.save(user).await
    }
}
