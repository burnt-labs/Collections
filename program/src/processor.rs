use crate::errors::CollectionError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, borsh::try_from_slice_unchecked, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey,
};
use std::mem;

pub mod add_members;
pub mod add_member_of;
pub mod arrange_member;
pub mod create_collection;
pub mod remove_member;

pub use add_members::*;
pub use add_member_of::*;
pub use arrange_member::*;
pub use create_collection::*;
pub use remove_member::*;

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
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct CollectionSignature {
    collection: Pubkey,
    signature: [u8; 32],
}

// todo(mvid): update this when struct finalized
pub const BASE_COLLECTION_DATA_SIZE: usize = 32 + 32 + 1 + 8;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct CollectionData {
    pub name: [u8; 32],
    pub description: [u8; 32],
    pub removable: bool,
    pub expandable: u32,
    pub arrangeable: bool,
    pub members: Vec<Pubkey>,
    pub member_of: Vec<CollectionSignature>,
}

impl CollectionData {
    // todo(mvid): don't deserialize entire collection
    // pub fn get_members(a: &AccountInfo) -> Vec<Pubkey> {
    //     let collection = CollectionData::from_account_info(a)?;
    // }

    pub fn from_account_info(a: &AccountInfo) -> Result<CollectionData, ProgramError> {
        if (a.data_len() - BASE_COLLECTION_DATA_SIZE) % mem::size_of::<Pubkey>() != 0 {
            return Err(CollectionError::DataTypeMismatch.into());
        }

        let collection: CollectionData = try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(collection)
    }
}
