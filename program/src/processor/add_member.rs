use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvar::{rent::Rent, Sysvar},
        system_instruction,
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
    rent: &'a AccountInfo<'b>,
    payer: &'a AccountInfo<'b>,
    system: &'a AccountInfo<'b>,
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
        rent: next_account_info(account_iter)?,
        payer: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
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

    let rent = &Rent::from_account_info(accounts.rent)?;
    // Additional lamports required to maintain rent exemption
    // TODO: need to get minimum required size based on addition of new collection member
    let required_additional_lamports = rent
        .minimum_balance(100)
        .saturating_sub(accounts.rent.lamports());

    if required_additional_lamports > 0 {
        msg!("{} additional lamports required to maintain rent exemption, transferring", required_additional_lamports);
        invoke(
            &system_instruction::transfer(&accounts.payer.key, &accounts.collection.key, required_additional_lamports),
            &[
                accounts.payer.clone(),
                accounts.collection.clone(),
                accounts.system.clone(),
            ],
        )?;
    }

    // TODO
    // Need to
    // 1. get signer seeds
    // 2. figure out current account space
    // 3. find out Solana's API for allocating additional account space. (is it fixed after account creation, like some docs suggest?)

    msg!("Allocate space for the account");
    invoke_signed(
        &system_instruction::allocate(accounts.collection.key, size.try_into().unwrap()),
        &[accounts.collection.clone()], // TODO: is this right?
        &[&signer_seeds],
    )?;

    // append the member to the collection
    collection.members.push(accounts.new_member.key.clone());
    collection.serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}
