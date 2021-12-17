use crate::errors::CollectionError;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        msg!(
            "{} Owner Invalid, Expected {}, Got {}",
            account.key,
            owner,
            account.owner
        );
        Err(CollectionError::IncorrectOwner.into())
    } else {
        Ok(())
    }
}

pub fn assert_authority(account: &AccountInfo, authorities: Vec<Pubkey>) -> ProgramResult {
    if !authorities.contains(account.key ) {
        msg!(
            "{} Authority Invalid",
            account.key,
        );
        return Err(CollectionError::InvalidAuthority.into());
    }

    if !account.is_signer {
        msg!(
            "{} Authority is not a signer",
            account.key,
        );
        return Err(CollectionError::AuthorityIsNotSigner.into());
    }

    Ok(())
}

#[inline(always)]
pub fn create_or_allocate_account_raw<'a>(
    program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> Result<(), ProgramError> {
    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamports > 0 {
        msg!("Transfer {} lamports to the new account", required_lamports);
        invoke(
            &system_instruction::transfer(&payer_info.key, new_account_info.key, required_lamports),
            &[
                payer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    let accounts = &[new_account_info.clone(), system_program_info.clone()];

    msg!("Allocate space for the account");
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        accounts,
        &[&signer_seeds],
    )?;

    msg!("Assign the account to the owning program");
    invoke_signed(
        &system_instruction::assign(new_account_info.key, &program_id),
        accounts,
        &[&signer_seeds],
    )?;
    msg!("Completed assignment!");

    Ok(())
}