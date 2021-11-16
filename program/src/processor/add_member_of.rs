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
    },
    utils::assert_owned_by
};
use crate::processor::CollectionSignature;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AddMemberOfArgs {
    // The member being added to the collection
    pub collection: Pubkey,
    pub signature: [u8; 32],
}

struct Accounts<'a, 'b: 'a> {
    collection: &'a AccountInfo<'b>,
    member_of_collection: &'a AccountInfo<'b>,
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
        member_of_collection: next_account_info(account_iter)?,
        payer: next_account_info(account_iter)?,
        rent: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
    };

    // assert the function is called by the collection owner
    assert_owned_by(accounts.collection, program_id)?;

    Ok(accounts)
}

pub fn add_member_of(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AddMemberOfArgs,
) -> ProgramResult {
    msg!("+ Processing AddMemberOf");
    let accounts = parse_accounts(program_id, accounts)?;

    // assert the collection can add members
    let mut collection = CollectionData::from_account_info(accounts.collection)?;

    let collection_signature = CollectionSignature{
        collection: accounts.member_of_collection.key(),
        signature: args.signature,
    };

    // append the collection to the parent collection
    collection.member_of.push(collection_signature);
    collection.serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}
