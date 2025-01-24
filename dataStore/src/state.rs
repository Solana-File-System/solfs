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

/// Verfies that the data conforms to the data_type
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
        DataStoreTypeOption::IMG => SerializationStatusOption::VERIFIED,
        DataStoreTypeOption::HTML => SerializationStatusOption::VERIFIED,
        DataStoreTypeOption::PDF => SerializationStatusOption::VERIFIED,
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct DataStoreAccountMetadata{
    data_status: DataStoreStatusOption,
    serialization_status: SerializationStatusOption,
    authority: Pubkey,
    is_dynamic: bool,
    data_type: DataStoreTypeOption,
    bump_seed: u8,
}

impl DataStoreAccountMetadata {
    pub fn new(authority: Pubkey, data_type: DataStoreTypeOption, bump_seed: u8) -> Self {
        Self {
            data_status: DataStoreStatusOption::Uninitialized,
            serialization_status: SerializationStatusOption::UNVERIFIED,
            authority,
            is_dynamic: false,
            data_type,
            bump_seed,
        }
    }
}

/// Get the Data Status
pub fn data_status(&self) -> DataStoreStatusOption {
    &self.data_status
}

/// set the Data Status
pub fn set_data_status(&mut self, data_status: DataStoreStatusOption) {
    self.data_status = data_status;
}

/// get serialization status
pub fn serialization_status(&self) -> SerializationStatusOption {
    &self.serialization_status
}

/// set serialization status
pub fn set_serialization_status(&mut self, serialization_status: SerializationStatusOption) {
    self.serialization_status = serialization_status;
}

/// get the authority
pub fn authority(&self) -> &Pubkey {
    &self.authority
}

/// set the authority
pub fn set_authority(&mut self, authority: Pubkey) {
    self.authority = authority;
}

/// get the is_dynamic
pub fn is_dynamic(&self) -> bool {
    &self.is_dynamic
}

/// get current data version
pub fn version(&self) -> u8 {
    self.data_version

}

/// get the data type
pub fn data_type(&self) -> &DataStoreTypeOption {
    &self.data_type
}

/// set the data type
pub fn set_data_type(&mut self, data_type: DataStoreTypeOption) {
    self.data_type = data_type;
}

/// get the bump seed
pub fn bump_seed(&self) -> u8 {
    self.bump_seed
}


#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct InitializeDataStoreArgs {
    pub authority: Pubkey,
    pub space: u64,
    pub is_dynamic: bool,
    pub is_created: bool,
    pub debug: bool,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct UpdateDataStoreArgs {
    pub data_type: DataStoreTypeOption,
    pub data: Vec<u8>,
    pub offset: u64,
    pub verify_data: bool,
    pub realloc_down: bool,
    pub debug: bool
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct CloseDataStoreArgs {
    pub debug: bool
}