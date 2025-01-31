use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

pub const METADATA_SIZE: usize = 1000;
pub const PDA_SEED: &[u8] = b"data_store";

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
#[borsh(use_discriminant = true)]
pub enum DataStoreTypeOption {
    File = 0,
    Directory = 1,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
#[borsh(use_discriminant = true)]
pub enum SerializationStatusOption {
    Uninitialized = 0,
    Initialized = 1,
    Finalized = 2,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct DataStoreAccountMetadata {
    pub data_type: DataStoreTypeOption,
    pub authority: Pubkey,
    pub data_status: SerializationStatusOption,
    pub bump_seed: u8,
    pub data_hash: [u8; 32],
    pub is_dynamic: bool,
    pub space: usize,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct InitializeDataStoreArgs {
    pub debug: bool,
    pub data_type: DataStoreTypeOption,
    pub bump_seed: u8,
    pub is_created: bool,
    pub space: u64,
    pub authority: Pubkey,
    pub is_dynamic: bool,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct UpdateDataStoreArgs {
    pub debug: bool,
    pub data_hash: [u8; 32],
    pub data: Vec<u8>,
    pub offset: u64,
    pub realloc_down: bool,
    pub data_type: DataStoreTypeOption,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct UpdateDataStoreAuthorityArgs {
    pub debug: bool,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct FinalizeDataStoreArgs {
    pub debug: bool,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct CloseDataStoreArgs {
    pub debug: bool,
}

impl DataStoreAccountMetadata {
    pub fn new(
        authority: Pubkey,
        data_type: DataStoreTypeOption,
        bump_seed: u8,
    ) -> Self {
        Self {
            data_type,
            authority,
            data_status: SerializationStatusOption::Initialized,
            bump_seed,
            data_hash: [0; 32],
            is_dynamic: false,
            space: 0,
        }
    }

    pub fn data_type(&self) -> &DataStoreTypeOption {
        &self.data_type
    }

    pub fn authority(&self) -> &Pubkey {
        &self.authority
    }

    pub fn data_status(&self) -> &SerializationStatusOption {
        &self.data_status
    }

    pub fn bump_seed(&self) -> u8 {
        self.bump_seed
    }

    pub fn data_hash(&self) -> &[u8; 32] {
        &self.data_hash
    }

    pub fn is_dynamic(&self) -> bool {
        self.is_dynamic
    }

    pub fn set_data_type(&mut self, data_type: DataStoreTypeOption) {
        self.data_type = data_type;
    }

    pub fn set_authority(&mut self, authority: &Pubkey) {
        self.authority = *authority;
    }

    pub fn set_data_status(&mut self, status: SerializationStatusOption) {
        self.data_status = status;
    }
}

/// Verifies that the data conforms to the data_type
pub fn verify_data(data: &[u8], data_type: DataStoreTypeOption) -> SerializationStatusOption {
    if data.is_empty() || data_type == DataStoreTypeOption::File {
        return SerializationStatusOption::Uninitialized;
    }
    match data_type {
        DataStoreTypeOption::Directory => SerializationStatusOption::Initialized,
        _ => SerializationStatusOption::Finalized,
    }
}