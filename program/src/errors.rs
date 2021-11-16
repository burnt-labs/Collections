use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that may be returned by the Collection program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum CollectionError {
    #[error("Collection account specified is invalid")]
    InvalidCollectionAccount,

    #[error("Collection does not have correct owner")]
    IncorrectOwner,

    #[error("Data type mismatch")]
    DataTypeMismatch,

    #[error("Collection not expandable")]
    NotExpandable,

    #[error("Collection not removable")]
    NotRemovable,

    #[error("Collection not removable")]
    NotArrangeable,

    #[error("Collection capacity exceeded")]
    CapacityExceeded,

    #[error("Permanently empty collection")]
    PermanentlyEmptyCollection,

    #[error("Permanently empty collection")]
    MemberAssetNotFound,

    #[error("Invalid original arrange index")]
    InvalidOriginalArrangeIndex,

    #[error("Invalid new arrange index")]
    InvalidNewArrangeIndex,
}


impl PrintProgramError for CollectionError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<CollectionError> for ProgramError {
    fn from(e: CollectionError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for CollectionError {
    fn type_of() -> &'static str {
        "Vault Error"
    }
}
