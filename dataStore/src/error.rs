use num_traits::FromPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum DataStoreError {
    #[error("Instruction is not implemented")]
    NotImplemented,
    #[error("Account should be writable")]
    NotWriteable,
    #[error("Account should not have zero-length data")]
    NoAccountLength,
    #[error("Account data length is invalid (non-zero data)")]
    NonZeroData,
    #[error("Account should be a signer")]
    NotSigner,
    #[error("Account should be a valid system program")]
    InvalidSystemProgram,
    #[error("Account should be a valid owner of the data store account")]
    InvalidAuthority,
    #[error("Account should be a valid PDA of the data account")]
    InvalidPDA,
    #[error("Cannot reinitialize a previously initialized data store account")]
    AlreadyInitialized,
    #[error("Data account should be initialized")]
    NotInitialized,
    #[error("Cannot update a previously finalized data store account")]
    AlreadyFinalized,
    #[error("Operation overflowed")]
    Overflow,
    #[error("Data account should have sufficient space")]
    InsufficientSpace,
    #[error("Invalid data type for verification")]
    InvalidDataType,
    #[error("Data verification failed")]
    DataVerificationFailed,
    #[error("Invalid instruction data")]
    InvalidInstructionData,
}

impl FromPrimitive for DataStoreError {
    fn from_i64(n: i64) -> Option<Self> {
        Self::from_u32(n as u32)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::from_u32(n as u32)
    }

    fn from_i32(n: i32) -> Option<Self> {
        Self::from_u32(n as u32)
    }

    fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::NotImplemented),
            1 => Some(Self::NotWriteable),
            2 => Some(Self::NoAccountLength),
            3 => Some(Self::NonZeroData),
            4 => Some(Self::NotSigner),
            5 => Some(Self::InvalidSystemProgram),
            6 => Some(Self::InvalidAuthority),
            7 => Some(Self::InvalidPDA),
            8 => Some(Self::AlreadyInitialized),
            9 => Some(Self::NotInitialized),
            10 => Some(Self::AlreadyFinalized),
            11 => Some(Self::Overflow),
            12 => Some(Self::InsufficientSpace),
            13 => Some(Self::InvalidDataType),
            14 => Some(Self::DataVerificationFailed),
            15 => Some(Self::InvalidInstructionData),
            _ => None,
        }
    }
}

impl From<DataStoreError> for ProgramError {
    fn from(e: DataStoreError) -> Self {
        ProgramError::Custom(e as u32)
    }
}