use crate::errors::CollectionError;
use bitflags::bitflags;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, borsh::try_from_slice_unchecked, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey,
};
use std::mem;

pub mod add_member;
pub mod add_member_of;
pub mod add_members;
pub mod arrange_member;
pub mod create_collection;
pub mod freeze_collection;
pub mod remove_member;
pub mod add_authority;
pub mod remove_authority;

pub use add_member::*;
pub use add_member_of::*;
pub use add_members::*;
pub use arrange_member::*;
pub use create_collection::*;
pub use freeze_collection::*;
pub use remove_member::*;
pub use add_authority::*;
pub use remove_authority::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    use crate::instruction::CollectionInstruction;
    match CollectionInstruction::try_from_slice(input)? {
        CollectionInstruction::CreateCollection(args) => {
            create_collection(program_id, accounts, args)
        }
        CollectionInstruction::AddMembers(args) => add_members(program_id, accounts, args),
        CollectionInstruction::RemoveMember(args) => remove_member(program_id, accounts, args),
        CollectionInstruction::ArrangeMember(args) => arrange_member(program_id, accounts, args),
        CollectionInstruction::AddMemberOf(args) => add_member_of(program_id, accounts, args),
        CollectionInstruction::FreezeCollection(args) => {
            freeze_collection(program_id, accounts, args)
        },
        CollectionInstruction::AddAuthority(args) => add_authority(program_id, accounts, args),
        CollectionInstruction::RemoveAuthority(args) => remove_authority(program_id, accounts, args),
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct CollectionSignature {
    collection: Pubkey,
    signature: [u8; 32],
}

pub const MAX_NAME_LENGTH: usize = 64;
pub const MAX_DESCRIPTION_LENGTH: usize = 512;
pub const MAX_IMAGE_LENGTH: usize = 2048;

// todo(mvid): update this when struct finalized
pub const BASE_COLLECTION_DATA_SIZE: usize = 1 // name
    + 32 // creator
    + 4 // authorities vec
    + 32 // initial authority
    + 1 // advanced
    + 4 // max_size
    + 4 // members vec
    + 4 // member_of vec
;

bitflags! {
    pub struct AdvancedOptions: u8 {
        const REMOVABLE   = 0b00000001;
        const EXPANDABLE  = 0b00000010;
        const ARRANGEABLE = 0b00000100;
    }
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct CollectionData {
    pub name: String,
    pub description: String,
    pub image: String,
    pub creator: Pubkey,
    pub authorities: Vec<Pubkey>,
    pub advanced: u8,
    pub max_size: u32,
    pub members: Vec<Pubkey>,
    pub member_of: Vec<CollectionSignature>,
}

impl CollectionData {
    pub fn from_account_info(a: &AccountInfo) -> Result<CollectionData, ProgramError> {
        if (a.data_len() - BASE_COLLECTION_DATA_SIZE) % mem::size_of::<Pubkey>() != 0 {
            return Err(CollectionError::DataTypeMismatch.into());
        }

        let collection: CollectionData = try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(collection)
    }
}
