use crate::errors::{AppError, AppResult};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserId(ObjectId);

impl UserId {
    pub fn new(id: ObjectId) -> Self {
        Self(id)
    }

    pub fn parse(s: &str) -> AppResult<Self> {
        ObjectId::parse_str(s)
            .map(Self)
            .map_err(|_| AppError::ValidationError(format!("Nieprawidłowy format UserId: {}", s)))
    }

    pub fn as_inner(&self) -> &ObjectId {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionToken(String);

impl SessionToken {
    pub fn new(token: String) -> AppResult<Self> {
        if token.trim().len() < 32 {
            return Err(AppError::ValidationError(
                "Token sesji jest zbyt krótki i niebezpieczny".into(),
            ));
        }
        Ok(Self(token))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionTtl(u64);

impl SessionTtl {
    pub fn from_secs(secs: u64) -> Self {
        Self(secs)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_parsing() {
        let valid_hex = "507f1f77bcf86cd799439011";
        let id = UserId::parse(valid_hex);
        assert!(id.is_ok());
        assert_eq!(id.unwrap().as_inner().to_hex(), valid_hex);

        let invalid_hex = "not-a-hex";
        assert!(UserId::parse(invalid_hex).is_err());
    }

    #[test]
    fn test_session_token_validation() {
        let short_token = "too-short".to_string();
        assert!(SessionToken::new(short_token).is_err());

        let long_token = "a".repeat(32);
        let token = SessionToken::new(long_token.clone());
        assert!(token.is_ok());
        assert_eq!(token.unwrap().as_str(), long_token);
    }

    #[test]
    fn test_session_ttl_value() {
        let ttl = SessionTtl::from_secs(3600);
        assert_eq!(ttl.as_u64(), 3600);
    }
}
