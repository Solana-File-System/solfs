use borsh::{BorshDeserialize, BorshSerialize};
use serde_json::Value;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

pub const DATA_STORE_VERSION: u8 = 0;
pub const METADATA_SIZE: usize = 1 + 1 + 32 + 1 + 1 + 1 + 1;
pub const PDA_SEED: &[u8] = b"data__store_account_metadata";

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum DataStoreState {
    Uninitialized,
    Initialized,
    Finalized,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum DataStoreTypeOption {
    CUSTOM = 0,
    JSON = 1,
    IMG = 2,
    HTML = 3,
    PDF = 4,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum SerializationStatusOption {
    UNVERIFIED,
    VERIFIED,
    FAILED,
}