use crate::{processor::CollectionData, utils::{assert_owned_by, assert_authority}};

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

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FreezeCollectionArgs {}

struct Accounts<'a, 'b: 'a> {
    collection: &'a AccountInfo<'b>,
    authority: &'a AccountInfo<'b>,
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let accounts = Accounts {
        collection: next_account_info(account_iter)?,
        authority: next_account_info(account_iter)?,
    };

    // assert the function is called by the collection owner
    assert_owned_by(accounts.collection, program_id)?;

    Ok(accounts)
}

pub fn freeze_collection(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _: FreezeCollectionArgs,
) -> ProgramResult {
    msg!("+ Processing FreezeCollection");
    let accounts = parse_accounts(program_id, accounts)?;

    let mut collection = CollectionData::from_account_info(accounts.collection)?;
    assert_authority(accounts.authority, collection.authorities.clone())?;

    // set all mutation options to false
    collection.advanced = 0;

    collection.serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}
