use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CryptoSettings {
    /// Główny Pepper (klucz serwera) dodawany do haseł
    pub secret_key: String,

    /// Czas życia tokenów JWT/Session
    pub token_expiry_hours: u64,

    /// Argon2: Koszt pamięciowy (np. 19456 dla 19MB - standard)
    pub argon2_m_cost: u32,

    /// Argon2: Liczba iteracji (np. 2)
    pub argon2_t_cost: u32,

    /// Argon2: Stopień równoległości (liczba wątków, np. 1)
    pub argon2_p_cost: u32,

    /// Długość generowanej soli w bajtach (standard 16)
    pub salt_len: usize,

    /// Długość Nonce dla AES-GCM (standard 12)
    pub nonce_len: usize,
}
