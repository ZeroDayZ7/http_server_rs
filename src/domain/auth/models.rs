pub struct UserId(pub String);
pub struct SessionToken(pub String);

#[derive(Debug, Clone, Copy)]
pub struct SessionTtl(pub u64);

impl From<&str> for UserId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
