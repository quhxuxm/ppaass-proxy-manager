use anyhow::Result;
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::LineEnding;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct RsaKeyPair {
    pub public_key_pem: String,
    pub private_key_pem: String,
}

pub fn generate_rsa_key_pair() -> Result<RsaKeyPair> {
    let private_key = RsaPrivateKey::new(&mut OsRng, 2048).expect("Fail to generate private key");
    let public_key = RsaPublicKey::from(&private_key);
    let private_key_pem = private_key
        .to_pkcs8_pem(LineEnding::CRLF)?;
    let public_key_pem = public_key
        .to_public_key_pem(LineEnding::CRLF)?;
    Ok(RsaKeyPair {
        public_key_pem,
        private_key_pem: private_key_pem.to_string(),
    })
}
