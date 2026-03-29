use crate::domain::VaultRepository;
use crate::domain::vault::{DecryptedCV, EncryptedCV};
use crate::errors::{AppError, AppResult};
use crate::utils::crypto::CryptoUtils;
use std::sync::Arc;
use tracing::{error, info, instrument};

pub struct VaultService {
    // Używamy Arc, aby móc współdzielić repozytorium między wątkami (wymagane przez Axum)
    repo: Arc<dyn VaultRepository + Send + Sync>,
}

impl VaultService {
    pub fn new(repo: Arc<dyn VaultRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    #[instrument(
        skip(self, access_key),
        fields(cv_id = %id)
    )]
    pub async fn unlock_cv(&self, id: &str, access_key: &str) -> AppResult<DecryptedCV> {
        info!("Pobieranie zaszyfrowanego CV z bazy");
        let encrypted = self.repo.get_cv_by_id(id).await?.ok_or_else(|| {
            error!("Nie znaleziono dokumentu o podanym ID");
            AppError::NotFound(format!("CV o ID {} nie istnieje", id))
        })?;

        // 2. Deszyfracja przy użyciu logiki krypto
        let decrypted_json = self.decrypt_logic(&encrypted, access_key)?;

        // 3. Deserializacja JSONa do struktury domenowej (DecryptedCV)
        let cv: DecryptedCV = serde_json::from_str(&decrypted_json).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Błąd formatu danych po deszyfracji: {}", e))
        })?;
        info!("CV pomyślnie odblokowane");
        Ok(cv)
    }

    #[instrument(skip(self, key, enc))]
    fn decrypt_logic(&self, enc: &EncryptedCV, key: &str) -> AppResult<String> {
        CryptoUtils::decrypt(
            &enc.data,  // Ciphertext z bazy (Base64)
            key,        // Hasło przekazane od użytkownika
            &enc.salt,  // Sól z bazy (Base64)
            &enc.nonce, // Nonce z bazy (Base64)
        )
    }
}
