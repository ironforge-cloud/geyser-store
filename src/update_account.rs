use serde::{Deserialize, Serialize};

use crate::{
    base64_encode, pubkey_string_from_bytes, tx_signature_string_from_bytes,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAccount {
    slot: u64,
    pubkey: Vec<u8>,
    lamports: u64,
    owner: Vec<u8>,
    executable: bool,
    rent_epoch: u64,
    data: Vec<u8>,
    write_version: u64,
    txn_signature: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateAccountStorable {
    pub slot: u64,
    pub pubkey: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
    pub data: Vec<u8>,
    pub write_version: u64,
    pub txn_signature: Option<String>,
}

impl std::fmt::Debug for UpdateAccountStorable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateAccountStorable")
            .field("slot", &self.slot)
            .field("pubkey", &self.pubkey)
            .field("lamports", &self.lamports)
            .field("owner", &self.owner)
            .field("executable", &self.executable)
            .field("rent_epoch", &self.rent_epoch)
            .field("data", &base64_encode(&self.data))
            .field("write_version", &self.write_version)
            .field("txn_signature", &self.txn_signature)
            .finish()
    }
}

impl From<UpdateAccount> for UpdateAccountStorable {
    fn from(item: UpdateAccount) -> Self {
        UpdateAccountStorable {
            slot: item.slot,
            pubkey: pubkey_string_from_bytes(&item.pubkey),
            lamports: item.lamports,
            owner: pubkey_string_from_bytes(&item.owner),
            executable: item.executable,
            rent_epoch: item.rent_epoch,
            data: item.data,
            write_version: item.write_version,
            txn_signature: item
                .txn_signature
                .map(|sig| tx_signature_string_from_bytes(&sig)),
        }
    }
}

impl UpdateAccountStorable {
    pub fn id(&self) -> String {
        format!("{} {:09}.{:03}", self.pubkey, self.slot, self.write_version)
    }
}
