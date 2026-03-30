// src/services/user_service.rs
use crate::domain::UserRepository;
use crate::domain::ports::services::UserServicePort;
use crate::domain::user::User;
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::instrument;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UserServicePort for UserService {
    #[instrument(skip(self), fields(user_email = %email))]
    async fn get_user_by_email(&self, email: &str) -> AppResult<User> {
        self.repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Użytkownik {} nie istnieje", email)))
    }

    #[instrument(skip(self, user), fields(user_id = ?user.id))]
    async fn register_user(&self, user: User) -> AppResult<()> {
        self.repo.save(user).await
    }
}
