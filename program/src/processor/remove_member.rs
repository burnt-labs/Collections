
use crate::{
    processor::{
        CollectionData,
        CollectionError
    },
    utils::assert_owned_by
};

use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        msg,
        pubkey::Pubkey,
        account_info::{AccountInfo, next_account_info},
        entrypoint::ProgramResult,
        program_error::ProgramError,
    },
};

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct RemoveMemberArgs {
    // The member being added to the collection
    pub asset: Pubkey,

    // The index of the asset in the member array
    pub index: usize,
}

struct Accounts<'a, 'b: 'a> {
    collection: &'a AccountInfo<'b>,
    removed_member: &'a AccountInfo<'b>,
    payer: &'a AccountInfo<'b>,
    rent: &'a AccountInfo<'b>,
    system: &'a AccountInfo<'b>,
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let accounts = Accounts {
        collection: next_account_info(account_iter)?,
        removed_member: next_account_info(account_iter)?,
        payer: next_account_info(account_iter)?,
        rent: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
    };

    // assert the function is called by the collection owner
    assert_owned_by(accounts.collection, program_id)?;

    Ok(accounts)
}


pub fn remove_member(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: RemoveMemberArgs,
) -> ProgramResult {
    msg!("+ Processing RemoveMember");
    let accounts = parse_accounts(program_id, accounts)?;

    // assert the collection can remove members
    let mut collection = CollectionData::from_account_info(accounts.collection)?;

    if !collection.removable {
        return Err(CollectionError::NotRemovable)
    } else {
        // assert the member asset at the index is the correct asset
        let asset_at_index = collection.members.get(args.index)?;
        if asset_at_index != args.asset {
            return Err(CollectionError::MemberAssetNotFound);
        }
    }

    // append the member to the collection
    collection.members.remove(args.index);
    collection.serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}
