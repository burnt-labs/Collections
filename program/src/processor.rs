use crate::errors::CollectionError;
use std::mem;
use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    borsh::try_from_slice_unchecked,
};

pub mod create_collection;
pub mod add_member;
pub mod remove_member;
pub mod arrange_member;
pub mod add_member_of;

pub use create_collection::*;
pub use add_member::*;
pub use remove_member::*;
pub use arrange_member::*;


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    use crate::instruction::CollectionInstruction;
    match CollectionInstruction::try_from_slice(input)? {
        CollectionInstruction::CreateCollection(args) => create_collection(program_id, accounts, args),
        CollectionInstruction::AddMember(args) => add_member(program_id, accounts, args),
        CollectionInstruction::RemoveMember(args) => remove_member(program_id, accounts, args),
        CollectionInstruction::ArrangeMember(args) => arrange_member(program_id, accounts, args),
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
    pub fn get_members(a: &AccountInfo) -> Vec<Pubkey> {
        let collection = CollectionData::from_account_info(a)?;
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<CollectionData, ProgramError> {
        if (a.data_len() - BASE_COLLECTION_DATA_SIZE) % mem::size_of::<Pubkey>() != 0 {
            return Err(CollectionError::DataTypeMismatch.into());
        }

        let collection: CollectionData = try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(collection)
    }
}
