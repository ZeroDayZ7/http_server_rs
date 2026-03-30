use crate::errors::app_error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, AppError> {
        let value = value.trim().to_lowercase();

        if value.is_empty() || !value.contains('@') {
            return Err(AppError::ValidationError("Invalid email format".into()));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
