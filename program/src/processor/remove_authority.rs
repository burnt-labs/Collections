use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::errors::CollectionError;
use crate::processor::CollectionData;
use crate::utils::{assert_authority, assert_owned_by};
use borsh::{BorshDeserialize, BorshSerialize};


#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct RemoveAuthorityArgs {}

struct Accounts<'a, 'b: 'a> {
    collection: &'a AccountInfo<'b>,
    authority: &'a AccountInfo<'b>,
    removed_authority: &'a AccountInfo<'b>,
}


fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let accounts = Accounts {
        collection: next_account_info(account_iter)?,
        authority: next_account_info(account_iter)?,
        removed_authority: next_account_info(account_iter)?,
    };

    // assert the function is called by the collection owner
    assert_owned_by(accounts.collection, program_id)?;

    Ok(accounts)
}

pub fn remove_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _: RemoveAuthorityArgs,
) -> ProgramResult {
    msg!("+ Processing RemoveAuthority");
    let accounts = parse_accounts(program_id, accounts)?;

    // assert the authority can modify this collection
    let mut collection = CollectionData::from_account_info(accounts.collection)?;
    assert_authority(accounts.authority, collection.authorities.clone())?;

    if !collection.authorities.contains(accounts.removed_authority.key) {
        return Err(CollectionError::AuthorityNotFound.into());
    }

    if collection.authorities.len() == 1 {
        return Err(CollectionError::CollectionMustHaveAuthority.into())
    }

    let authority_index = collection.authorities.iter().position(|x| x == accounts.removed_authority.key).unwrap();
    collection.authorities.swap_remove(authority_index);
    collection.serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}