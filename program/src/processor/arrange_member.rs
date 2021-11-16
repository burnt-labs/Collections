use {
    solana_program::{
        account_info::{
            AccountInfo,
            next_account_info,
        },
        msg,
        pubkey::Pubkey,
        program_error::ProgramError,
        entrypoint::ProgramResult,
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

use crate::{
    processor::{
        CollectionData,
        CollectionError
    },
    utils::assert_owned_by
};

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ArrangeMemberArgs {
    // The original index of the member asset being arranged
    pub old_index: usize,

    // The new index of the member asset being arranged
    pub new_index: usize,
}

struct Accounts<'a, 'b: 'a> {
    collection: &'a AccountInfo<'b>,
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
        payer: next_account_info(account_iter)?,
        rent: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
    };

    // assert the function is called by the collection owner
    assert_owned_by(accounts.collection, program_id)?;

    Ok(accounts)
}


pub fn arrange_member(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ArrangeMemberArgs,
) -> ProgramResult {
    msg!("+ Processing ArrangeMember");
    let accounts = parse_accounts(program_id, accounts)?;

    if args.old_index == args.new_index {
        return Ok(())
    }

    // assert the collection can add members
    let mut collection = CollectionData::from_account_info(accounts.collection)?;

    if !collection.arrangeable {
        return Err(CollectionError::NotArrangeable)
    } else if args.old_index >= collection.members.len() {
        return Err(CollectionError::InvalidOriginalArrangeIndex)
    } else if args.new_index >= collection.members.len() {
        return Err(CollectionError::InvalidNewArrangeIndex)
    }

    // arrange the member in the collection
    collection.members.swap(args.old_index, args.old_index);
    collection.serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}
