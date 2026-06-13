use anyhow::{anyhow, Result};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};

use crate::domain::traits::service::CryptoService;

#[derive(Clone)]
pub struct Ed25519CryptoService;

impl Ed25519CryptoService {
    pub fn new() -> Self {
        Self
    }
}

impl CryptoService for Ed25519CryptoService {
    fn generate_keypair(&self) -> Result<(String, String)> {
        let mut csprng = rand::thread_rng();
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let private_key_hex = hex::encode(signing_key.to_bytes());
        let public_key_hex = hex::encode(verifying_key.to_bytes());

        Ok((private_key_hex, public_key_hex))
    }

    fn sign_payload(&self, private_key_hex: &str, payload: &[u8]) -> Result<String> {
        let key_bytes = hex::decode(private_key_hex)
            .map_err(|e| anyhow!("Invalid private key hex: {}", e))?;

        let key_array: [u8; 32] = key_bytes
            .try_into()
            .map_err(|_| anyhow!("Private key must be 32 bytes"))?;

        let signing_key = SigningKey::from_bytes(&key_array);
        let signature = signing_key.sign(payload);

        Ok(hex::encode(signature.to_bytes()))
    }

    fn verify_signature(
        &self,
        public_key_hex: &str,
        payload: &[u8],
        signature_hex: &str,
    ) -> Result<bool> {
        let key_bytes = hex::decode(public_key_hex)
            .map_err(|e| anyhow!("Invalid public key hex: {}", e))?;

        let key_array: [u8; 32] = key_bytes
            .try_into()
            .map_err(|_| anyhow!("Public key must be 32 bytes"))?;

        let verifying_key = VerifyingKey::from_bytes(&key_array)
            .map_err(|e| anyhow!("Invalid public key: {}", e))?;

        let sig_bytes = hex::decode(signature_hex)
            .map_err(|e| anyhow!("Invalid signature hex: {}", e))?;

        let sig_array: [u8; 64] = sig_bytes
            .try_into()
            .map_err(|_| anyhow!("Signature must be 64 bytes"))?;

        let signature = ed25519_dalek::Signature::from_bytes(&sig_array);

        match verifying_key.verify(payload, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
