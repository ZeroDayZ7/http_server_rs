// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

pub mod mongo_user_repo;
pub mod user_repo;

use crate::domain::user::User;
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn save(&self, user: User) -> AppResult<()>;
}
