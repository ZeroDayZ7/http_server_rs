use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedCV {
    pub id: String,
    pub data: String,
    pub salt: String,
    pub nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptedCV {
    pub name: String,
    pub experience: Vec<String>,
    pub contact: String,
}
