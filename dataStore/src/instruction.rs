use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankInstruction, account};

use crate::state::{
    CloseDataStoreArgs, FinalizeDataStoreArgs, InitializeDataStoreArgs, UpdateDataStoreArgs,
    UpdateDataStoreAuthorityArgs,
};

/// Instructions supported by the Data Store.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum DataStoreInstruction {
    /// Initializes a new data store account. that is accessible by the authority.
    /// sets the owner of the data store account to be the data program.
    /// If a data account was already initialized for given user, it returns Error.
    #[account(0, signer, writable, name = "feepayer", desc = "Account responsible for paying the transaction fees for initializing the data store")]
    #[account(1, writable, name = "datastore", desc = "Data store account")] 
    #[account(2, writable, name = "data_store_pda", desc = "Data Store pda's account")]
    #[account(3, name = "system_program", desc = "System program account")]
    InitializeDataStore(InitializeDataStoreArgs),

    /// Updates the data store account.
    #[account(0, signer, writable, name = "authority", desc = "Authority account")]
    #[account(1, writable, name = "datastore", desc = "Data store account")]
    #[account(2, writable, name = "data_store_pda", desc = "Data Store pda's account")]
    #[account(3, name = "system_program", desc = "System program account")]
    UpdateDataStore(UpdateDataStoreArgs),


    /// Updates the authority of the data store account.
    #[account(0, signer, name = "old_authority", desc = "Old Authority account")]
    #[account(1, signer, name = "datastore", desc = "Data account")]
    #[account(2, writable, name = "data_store_pda", desc = "Data Store pda's account")]
    #[account(3, name = "new_authority", desc = "New Authority account")]
    UpdateDataStoreAuthority(UpdateDataStoreAuthorityArgs),


    /// Finalizes the data store account.
    #[account(0, signer, name = "authority", desc = "Authority account")]
    #[account(1, name = "datastore", desc = "Data store account")]
    #[account(2, writable, name = "data_store_pda", desc = "Data Store pda's account")]
    FinalizeDataStore(FinalizeDataStoreArgs),

    /// Closes the data store account.
    #[account(0, signer, name = "authority", desc = "Authority account")]
    #[account(1, writable, name = "datastore", desc = "Data store account")]
    #[account(2, writable, name = "data_store_pda", desc = "Data Store pda's account")]
    CloseDataStore(CloseDataStoreArgs),
}