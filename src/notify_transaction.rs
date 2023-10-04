use crate::{
    events::{
        SerializableSanitizedTransaction, SerializableTransactionStatusMeta,
    },
    SerializableTransactionEvent,
};
use serde::{Deserialize, Serialize};

use crate::tx_signature_string_from_bytes;

// -----------------
// TransactionStorable
// -----------------
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStorable {
    pub signature: String,
    pub is_vote: bool,
    pub slot: u64,
    pub index: u64,
    pub transaction: Option<SerializableSanitizedTransaction>,
    pub transaction_status_meta: Option<SerializableTransactionStatusMeta>,
}

impl From<SerializableTransactionEvent> for TransactionStorable {
    fn from(event: SerializableTransactionEvent) -> Self {
        let signature = tx_signature_string_from_bytes(&event.signature);

        Self {
            signature,
            is_vote: event.is_vote,
            slot: event.slot,
            index: event.index,
            transaction: event.transaction,
            transaction_status_meta: event.transaction_status_meta,
        }
    }
}
