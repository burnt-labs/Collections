use crate::{
    processor::{CollectionData, CollectionError},
    utils::assert_owned_by,
};

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
use crate::utils::assert_authority;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct RemoveMemberArgs {
    // The index of the asset in the member array
    pub index: usize,
}

struct Accounts<'a, 'b: 'a> {
    collection: &'a AccountInfo<'b>,
    authority: &'a AccountInfo<'b>,
    removed_member: &'a AccountInfo<'b>,
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let accounts = Accounts {
        collection: next_account_info(account_iter)?,
        authority: next_account_info(account_iter)?,
        removed_member: next_account_info(account_iter)?,
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
    assert_authority(accounts.authority, collection.authorities.clone())?;

    if !collection.advanced.to_le_bytes()[0] == 1 {
        return Err(CollectionError::NotRemovable.into());
    } else {
        // assert the member asset at the index is the correct asset
        let asset_at_index = collection.members.get(args.index).unwrap();
        if *asset_at_index != *accounts.removed_member.key {
            return Err(CollectionError::MemberAssetNotFound.into());
        }
    }

    // append the member to the collection
    collection.members.remove(args.index);
    collection.serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}
