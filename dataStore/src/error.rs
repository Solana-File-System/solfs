use num_derive::FromPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum DataStoreError { 
    #[error("Instruction is not implemented")]
    NotImplemented,
    #[error("Account should be writeable")]
    NotWriteable,
    #[error("Account should not have zero length data")]
    NoAccountLength,
    #[error("Account data length is invalid (non-zero data)")]
    NonZeroData,
    #[error("Account should be a signer)")]
    NotSigner,
    #[error("Account should be valid system program")]
    InvalidSystemProgram,
    #[error("Account should be valid owner of data store account")]
    InvalidAuthority,
    #[error("Account should be PDA of data account")]
    InvalidPDA,
    #[error("Cannot reinitialize previously initialized data store account")]
    AlreadyInitialized,
    #[error("Data account should be initialized")]
    NotInitialized,
    #[error("Cannot update previously finalized data store account")]
    AlreadyFinalized,
    #[error("Operation overflowed")]
    Overflow,
    #[error("Data account should have sufficient space")]
    InsufficientSpace,
}

impl From<DataStoreError> for ProgramError {
    fn from(e: DataStoreError) -> Self {
        ProgramError::Custom(e as u32)
    }
}