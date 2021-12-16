use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

use crate::{
    processor::{AdvancedOptions, CollectionData, CollectionError},
    utils::assert_owned_by,
};
use crate::utils::assert_authority;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AddMemberArgs {}

struct Accounts<'a, 'b: 'a> {
    collection: &'a AccountInfo<'b>,
    authority: &'a AccountInfo<'b>,
    new_member: &'a AccountInfo<'b>,
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let accounts = Accounts {
        collection: next_account_info(account_iter)?,
        authority: next_account_info(account_iter)?,
        new_member: next_account_info(account_iter)?,
    };

    // assert the function is called by the collection owner
    assert_owned_by(accounts.collection, program_id)?;

    Ok(accounts)
}

pub fn add_member(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _: AddMemberArgs,
) -> ProgramResult {
    msg!("+ Processing AddMember");
    let accounts = parse_accounts(program_id, accounts)?;

    // assert the collection can add members
    let mut collection = CollectionData::from_account_info(accounts.collection)?;
    assert_authority(accounts.authority, collection.authorities.clone())?;

    let options = AdvancedOptions::from_bits(collection.advanced).unwrap();
    if (options & AdvancedOptions::EXPANDABLE) != AdvancedOptions::EXPANDABLE {
        return Err(CollectionError::NotExpandable.into());
    } else if collection.max_size as usize == collection.members.len() {
        return Err(CollectionError::CapacityExceeded.into());
    }

    // append the member to the collection
    collection.members.push(accounts.new_member.key.clone());
    collection.serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}
