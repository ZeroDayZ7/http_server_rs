// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use crate::domain::user::User;
use crate::errors::{AppError, AppResult};
use crate::repository::UserRepository;
use std::sync::Arc;

pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn get_user_by_email(&self, email: &str) -> AppResult<User> {
        self.repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Użytkownik {} nie istnieje", email)))
    }

    pub async fn register_user(&self, user: User) -> AppResult<()> {
        // Tu docelowo dojdzie hashowanie hasła przed save()
        self.repo.save(user).await
    }
}
