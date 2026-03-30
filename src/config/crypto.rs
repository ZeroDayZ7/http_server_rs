use serde::{Deserialize, Deserializer};

// --- TYPY DEDYKOWANE (NewTypes) ---

#[derive(Debug, Clone, Copy)]
pub struct SaltLength(pub usize);

impl<'de> Deserialize<'de> for SaltLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = usize::deserialize(deserializer)?;
        if val < 8 || val > 64 {
            return Err(serde::de::Error::custom(
                "salt_len must be between 8 and 64 bytes",
            ));
        }
        Ok(SaltLength(val))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NonceLength(pub usize);

impl<'de> Deserialize<'de> for NonceLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = usize::deserialize(deserializer)?;
        if val != 12 {
            return Err(serde::de::Error::custom(
                "nonce_len for AES-GCM must be exactly 12 bytes",
            ));
        }
        Ok(NonceLength(val))
    }
}

// --- GŁÓWNA STRUKTURA ---

#[derive(Debug, Deserialize, Clone)]
pub struct CryptoSettings {
    pub secret_key: String,
    pub token_expiry_hours: u64,

    pub argon2_m_cost: u32,
    pub argon2_t_cost: u32,
    pub argon2_p_cost: u32,

    pub salt_len: SaltLength,
    pub nonce_len: NonceLength,
}
