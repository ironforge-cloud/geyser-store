use base64::{engine::general_purpose, Engine as _};
use solana_sdk::{pubkey::Pubkey, signature::Signature};

use crate::errors::GeyserStoreResult;

pub fn pubkey_string_from_bytes(bytes: &[u8]) -> String {
    let pubkey = Pubkey::try_from(bytes).unwrap();
    pubkey.to_string()
}

pub fn tx_signature_string_from_bytes(bytes: &[u8]) -> String {
    let signature = Signature::try_from(bytes).unwrap();
    signature.to_string()
}

pub fn base64_decode(data: &str) -> GeyserStoreResult<Vec<u8>> {
    Ok(general_purpose::STANDARD.decode(data)?)
}

pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}
