// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use crate::domain::user::User;
use mongodb::Database;
use mongodb::bson::doc;
use std::sync::Arc;

pub struct UserRepository {
    db: Arc<Database>,
}

impl UserRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn find_user_by_email(&self, email: &str) -> mongodb::error::Result<Option<User>> {
        let collection = self.db.collection::<User>("users");
        collection.find_one(doc! { "email": email }).await
    }
}
