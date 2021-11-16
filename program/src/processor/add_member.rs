use {
    solana_program::{
        account_info::{
            AccountInfo,
            next_account_info,
        },
        msg,
        pubkey::Pubkey,
        entrypoint::ProgramResult,
        program_error::ProgramError,
    },
    borsh::{BorshDeserialize, BorshSerialize}
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
pub struct AddMemberArgs {
    // The member being added to the collection
    pub asset: Pubkey,
}

struct Accounts<'a, 'b: 'a> {
    collection: &'a AccountInfo<'b>,
    new_member: &'a AccountInfo<'b>,
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
        new_member: next_account_info(account_iter)?,
        payer: next_account_info(account_iter)?,
        rent: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
    };

    // assert the function is called by the collection owner
    assert_owned_by(accounts.collection, program_id)?;

    Ok(accounts)
}

pub fn add_member(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AddMemberArgs,
) -> ProgramResult {
    msg!("+ Processing AddMember");
    let accounts = parse_accounts(program_id, accounts)?;

    // assert the collection can add members
    let mut collection = CollectionData::from_account_info(accounts.collection)?;

    if collection.expandable == 0 {
        return Err(CollectionError::NotExpandable)
    } else if collection.expandable == collection.members.len() {
        return Err(CollectionError::CapacityExceeded)
    }

    // append the member to the collection
    collection.members.push(accounts.new_member.key);
    collection.serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}
