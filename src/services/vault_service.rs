use crate::domain::vault::{DecryptedCV, EncryptedCV, VaultRepository};
use crate::errors::{AppError, AppResult};
use crate::utils::crypto::CryptoUtils;
use std::sync::Arc;

pub struct VaultService {
    // Używamy Arc, aby móc współdzielić repozytorium między wątkami (wymagane przez Axum)
    repo: Arc<dyn VaultRepository + Send + Sync>,
}

impl VaultService {
    pub fn new(repo: Arc<dyn VaultRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    /// Główna metoda odblokowująca dane CV
    pub async fn unlock_cv(&self, id: &str, access_key: &str) -> AppResult<DecryptedCV> {
        // 1. Pobranie zaszyfrowanych danych z repozytorium
        let encrypted = self
            .repo
            .get_cv_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("CV o ID {} nie istnieje", id)))?;

        // 2. Deszyfracja przy użyciu logiki krypto
        let decrypted_json = self.decrypt_logic(&encrypted, access_key)?;

        // 3. Deserializacja JSONa do struktury domenowej (DecryptedCV)
        let cv: DecryptedCV = serde_json::from_str(&decrypted_json).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Błąd formatu danych po deszyfracji: {}", e))
        })?;

        Ok(cv)
    }

    /// Logika łącząca dane z bazy z narzędziami krypto
    fn decrypt_logic(&self, enc: &EncryptedCV, key: &str) -> AppResult<String> {
        // Poprawione mapowanie pól zgodnie z Twoim modelem:
        // enc.data zamiast enc.encrypted_data
        CryptoUtils::decrypt(
            &enc.data,  // Ciphertext z bazy (Base64)
            key,        // Hasło przekazane od użytkownika
            &enc.salt,  // Sól z bazy (Base64)
            &enc.nonce, // Nonce z bazy (Base64)
        )
    }
}
