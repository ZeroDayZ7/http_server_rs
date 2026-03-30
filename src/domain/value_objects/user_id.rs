use crate::errors::app_error::AppError;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(ObjectId);

impl UserId {
    pub fn new(id: ObjectId) -> Self {
        Self(id)
    }

    pub fn parse(value: &str) -> Result<Self, AppError> {
        ObjectId::parse_str(value)
            .map(Self)
            .map_err(|_| AppError::ValidationError(format!("Invalid UserId: {}", value)))
    }

    pub fn as_inner(&self) -> &ObjectId {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hex())
    }
}
