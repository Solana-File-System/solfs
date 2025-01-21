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

/// Verifies that the data conforms to the data_type
pub fn verify_data(data: &[u8], data_type: DataStoreTypeOption) -> SerializationStatusOption {
    if data.is_empty() || data_type == DataStoreTypeOption::CUSTOM {
        return SerializationStatusOption::UNVERIFIED;
    }
    match data_type {
        DataStoreTypeOption::JSON => {
            match serde_json::from_slice::<Value>(data) {
                Ok(_) => SerializationStatusOption::VERIFIED,
                Err(_) => SerializationStatusOption::FAILED,
            }
        }
        _ => SerializationStatusOption::VERIFIED,
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct DataStoreAccountMetadata {
    data_status: DataStoreState,
    serialization_status: SerializationStatusOption,
    authority: Pubkey,
    is_dynamic: bool,
    data_type: DataStoreTypeOption,
    bump_seed: u8,
}

impl DataStoreAccountMetadata {
    pub fn new(authority: Pubkey, data_type: DataStoreTypeOption, bump_seed: u8) -> Self {
        Self {
            data_status: DataStoreState::Uninitialized,
            serialization_status: SerializationStatusOption::UNVERIFIED,
            authority,
            is_dynamic: false,
            data_type,
            bump_seed,
        }
    }

    pub fn data_status(&self) -> &DataStoreState {
        &self.data_status
    }

    pub fn set_data_status(&mut self, data_status: DataStoreState) {
        self.data_status = data_status;
    }

    pub fn serialization_status(&self) -> &SerializationStatusOption {
        &self.serialization_status
    }

    pub fn set_serialization_status(&mut self, serialization_status: SerializationStatusOption) {
        self.serialization_status = serialization_status;
    }

    pub fn authority(&self) -> &Pubkey {
        &self.authority
    }

    pub fn set_authority(&mut self, authority: Pubkey) {
        self.authority = authority;
    }

    pub fn is_dynamic(&self) -> bool {
        self.is_dynamic
    }

    pub fn data_type(&self) -> &DataStoreTypeOption {
        &self.data_type
    }

    pub fn set_data_type(&mut self, data_type: DataStoreTypeOption) {
        self.data_type = data_type;
    }

    pub fn bump_seed(&self) -> u8 {
        self.bump_seed
    }
}