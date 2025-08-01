use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct AuthToken {
    object_id: Uuid,
    object: Value,
    exp: i64,
}

impl AuthToken {
    fn new(object_id: Uuid, object: Value, time_to_live: i64) -> Self {
        let exp = chrono::Utc::now().timestamp() + time_to_live;
        AuthToken {
            object_id,
            object,
            exp,
        }
    }

    fn to_string(&self, key: &Key<Aes256Gcm>) -> String {
        let token = serde_json::to_string(self).unwrap();
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(b"unique nonce"); // Use a unique nonce for each token
        let encrypted = cipher.encrypt(nonce, token.as_bytes()).unwrap();
        general_purpose::STANDARD.encode(encrypted)
    }

    fn get(&self, key: String) -> Option<&Value> {
        self.object.get(&key)
    }
}
