#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountEvent {
    /// The slot number when this update was emitted.
    #[prost(uint64, tag = "1")]
    pub slot: u64,
    /// The Pubkey for the account.
    #[prost(bytes = "vec", tag = "2")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    /// The lamports held by the account.
    #[prost(uint64, tag = "3")]
    pub lamports: u64,
    /// The Pubkey of the owner program account.
    #[prost(bytes = "vec", tag = "4")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    /// This account's data contains a loaded program.
    #[prost(bool, tag = "5")]
    pub executable: bool,
    /// The epoch at which this account will next owe rent.
    #[prost(uint64, tag = "6")]
    pub rent_epoch: u64,
    /// The data held in this account.
    #[prost(bytes = "vec", tag = "7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// A global monotonically increasing atomic number, which can be used
    /// to tell the order of the account update. For example, when an
    /// account is updated in the same slot multiple times, the update
    /// with higher write_version should supersede the one with lower
    /// write_version.
    #[prost(uint64, tag = "8")]
    pub write_version: u64,
    /// First signature of the transaction caused this account modification
    #[prost(bytes = "vec", optional, tag = "9")]
    pub txn_signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotStatusEvent {
    #[prost(uint64, tag = "1")]
    pub slot: u64,
    #[prost(uint64, tag = "2")]
    pub parent: u64,
    #[prost(enumeration = "SlotStatus", tag = "3")]
    pub status: i32,
}
/// MessageHeader
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageHeader {
    #[prost(uint32, tag = "1")]
    pub num_required_signatures: u32,
    #[prost(uint32, tag = "2")]
    pub num_readonly_signed_accounts: u32,
    #[prost(uint32, tag = "3")]
    pub num_readonly_unsigned_accounts: u32,
}
/// CompiledInstruction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompiledInstruction {
    #[prost(uint32, tag = "1")]
    pub program_id_index: u32,
    #[prost(uint32, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadedAddresses {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub writable: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub readonly: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageAddressTableLookup {
    #[prost(bytes = "vec", tag = "1")]
    pub account_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, repeated, tag = "2")]
    pub writable_indexes: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "3")]
    pub readonly_indexes: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct V0Message {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<MessageHeader>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub account_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "3")]
    pub recent_block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "4")]
    pub instructions: ::prost::alloc::vec::Vec<CompiledInstruction>,
    #[prost(message, repeated, tag = "5")]
    pub address_table_lookup:
        ::prost::alloc::vec::Vec<MessageAddressTableLookup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct V0LoadedMessage {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<V0Message>,
    #[prost(message, optional, tag = "2")]
    pub loaded_adresses: ::core::option::Option<LoadedAddresses>,
    #[prost(bool, repeated, tag = "3")]
    pub is_writable_account_cache: ::prost::alloc::vec::Vec<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyMessage {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<MessageHeader>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub account_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "3")]
    pub recent_block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "4")]
    pub instructions: ::prost::alloc::vec::Vec<CompiledInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyLoadedMessage {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<LegacyMessage>,
    #[prost(bool, repeated, tag = "2")]
    pub is_writable_account_cache: ::prost::alloc::vec::Vec<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SanitizedMessage {
    #[prost(oneof = "sanitized_message::MessagePayload", tags = "1, 2")]
    pub message_payload:
        ::core::option::Option<sanitized_message::MessagePayload>,
}
/// Nested message and enum types in `SanitizedMessage`.
pub mod sanitized_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MessagePayload {
        #[prost(message, tag = "1")]
        Legacy(super::LegacyLoadedMessage),
        #[prost(message, tag = "2")]
        V0(super::V0LoadedMessage),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SanitizedTransaction {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<SanitizedMessage>,
    #[prost(bytes = "vec", tag = "2")]
    pub message_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "3")]
    pub is_simple_vote_transaction: bool,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// <https://github.com/solana-labs/solana/pull/28430/files>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerInstructions {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(message, repeated, tag = "2")]
    pub instructions: ::prost::alloc::vec::Vec<InnerInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerInstruction {
    #[prost(message, optional, tag = "1")]
    pub instruction: ::core::option::Option<CompiledInstruction>,
    #[prost(uint32, optional, tag = "2")]
    pub stack_height: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiTokenAmount {
    #[prost(message, optional, tag = "1")]
    pub ui_amount: ::core::option::Option<f64>,
    #[prost(uint32, tag = "2")]
    pub decimals: u32,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ui_amount_string: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionTokenBalance {
    #[prost(uint32, tag = "1")]
    pub account_index: u32,
    #[prost(string, tag = "2")]
    pub mint: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub ui_token_account: ::core::option::Option<UiTokenAmount>,
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reward {
    #[prost(string, tag = "1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub lamports: i64,
    #[prost(uint64, tag = "3")]
    pub post_balance: u64,
    /// reward_type is an enum, but protobuf will require it to be able to accept any int32.
    #[prost(int32, tag = "4")]
    pub reward_type: i32,
    #[prost(uint32, tag = "5")]
    pub commission: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionStatusMeta {
    #[prost(bool, tag = "1")]
    pub is_status_err: bool,
    #[prost(string, tag = "2")]
    pub error_info: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub fee: u64,
    #[prost(uint64, repeated, tag = "4")]
    pub pre_balances: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "5")]
    pub post_balances: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag = "6")]
    pub inner_instructions: ::prost::alloc::vec::Vec<InnerInstructions>,
    #[prost(string, repeated, tag = "7")]
    pub log_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "8")]
    pub pre_token_balances: ::prost::alloc::vec::Vec<TransactionTokenBalance>,
    #[prost(message, repeated, tag = "9")]
    pub post_token_balances: ::prost::alloc::vec::Vec<TransactionTokenBalance>,
    #[prost(message, repeated, tag = "10")]
    pub rewards: ::prost::alloc::vec::Vec<Reward>,
}
/// based on solana_accountsdb_plugin_interface::accountsdb_plugin_interface::ReplicaTransactionInfo
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionEvent {
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub is_vote: bool,
    #[prost(message, optional, tag = "3")]
    pub transaction: ::core::option::Option<SanitizedTransaction>,
    #[prost(message, optional, tag = "4")]
    pub transaction_status_meta: ::core::option::Option<TransactionStatusMeta>,
    #[prost(uint64, tag = "5")]
    pub slot: u64,
    #[prost(uint64, tag = "6")]
    pub index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageWrapper {
    #[prost(oneof = "message_wrapper::EventMessage", tags = "1, 2, 3")]
    pub event_message: ::core::option::Option<message_wrapper::EventMessage>,
}
/// Nested message and enum types in `MessageWrapper`.
pub mod message_wrapper {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EventMessage {
        #[prost(message, tag = "1")]
        Account(::prost::alloc::boxed::Box<super::UpdateAccountEvent>),
        #[prost(message, tag = "2")]
        Slot(::prost::alloc::boxed::Box<super::SlotStatusEvent>),
        #[prost(message, tag = "3")]
        Transaction(::prost::alloc::boxed::Box<super::TransactionEvent>),
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum SlotStatus {
    /// The highest slot of the heaviest fork processed by the node. Ledger state at this slot is
    /// not derived from a confirmed or finalized block, but if multiple forks are present, is from
    /// the fork the validator believes is most likely to finalize.
    Processed = 0,
    /// The highest slot having reached max vote lockout.
    Rooted = 1,
    /// The highest slot that has been voted on by supermajority of the cluster, ie. is confirmed.
    Confirmed = 2,
}
impl SlotStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SlotStatus::Processed => "Processed",
            SlotStatus::Rooted => "Rooted",
            SlotStatus::Confirmed => "Confirmed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Processed" => Some(Self::Processed),
            "Rooted" => Some(Self::Rooted),
            "Confirmed" => Some(Self::Confirmed),
            _ => None,
        }
    }
}
