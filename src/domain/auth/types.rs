// src/domain/auth/types.rs
pub struct UserId(pub String);
pub struct SessionToken(pub String);

impl From<String> for UserId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for SessionToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
