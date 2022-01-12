use crate::processor::{
    AdvancedOptions, CollectionData, CollectionSignature, BASE_COLLECTION_DATA_SIZE,
};
use crate::{errors::CollectionError, utils::create_or_allocate_account_raw, PREFIX};
use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
        program_error::ProgramError, pubkey::Pubkey,
    },
    std::mem,
};

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateCollectionArgs {
    // The name of the Collection
    pub name: String,
    // A short description of the Collection
    pub description: String,
    // A url for the collection to display
    pub image: String,
    // A u8 storing boolean values for advanced options removable, expandable, arrangeable
    // removable >> 1: whether assets can be removed from the `members` list
    // expandable >> 2: if assets can be appended to the collection
    // arrangeable >> 3: whether asset order can be changed
    pub advanced: u8,
    // A u32 that declares what the maximum number of member assets on the chain can be.
    // If set to 0, the collection has no max size.
    pub max_size: u32,
    // A list of public keys that this collection considers to be members
    pub members: Vec<Pubkey>,
    // A list of signature that this collection is a member of
    pub member_of: Vec<CollectionSignature>,
}

struct Accounts<'a, 'b: 'a> {
    collection: &'a AccountInfo<'b>,
    creator: &'a AccountInfo<'b>,
    payer: &'a AccountInfo<'b>,
    rent: &'a AccountInfo<'b>,
    system: &'a AccountInfo<'b>,
}

fn parse_accounts<'a, 'b: 'a>(
    _: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let accounts = Accounts {
        collection: next_account_info(account_iter)?,
        creator: next_account_info(account_iter)?,
        payer: next_account_info(account_iter)?,
        rent: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
    };

    if !accounts.creator.is_signer {
        return Err(CollectionError::CreatorIsNotSigner.into());
    }

    Ok(accounts)
}

pub fn create_collection(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateCollectionArgs,
) -> ProgramResult {
    msg!("+ Processing CreateCollection");
    let accounts = parse_accounts(program_id, accounts)?;

    let collection_path = [
        PREFIX.as_bytes(),
        program_id.as_ref(),
        accounts.creator.key.as_ref(),
        args.name.as_bytes(),
    ];

    let (collection_key, bump) = Pubkey::find_program_address(&collection_path, program_id);
    if collection_key != *accounts.collection.key {
        return Err(CollectionError::InvalidCollectionAccount.into());
    }

    let options = AdvancedOptions::from_bits(args.advanced).unwrap();
    let mut account_size = BASE_COLLECTION_DATA_SIZE;
    if args.max_size > 0 {
        if args.members.len() > args.max_size as usize {
            return Err(CollectionError::CapacityExceeded.into());
        }

        account_size += args.max_size as usize * mem::size_of::<Pubkey>();
    } else if (options & AdvancedOptions::EXPANDABLE) == AdvancedOptions::EXPANDABLE {
        if args.members.len() == 0 {
            return Err(CollectionError::PermanentlyEmptyCollection.into());
        }
        account_size += args.members.len() * mem::size_of::<Pubkey>();
    }

    if args.member_of.len() > 0 {
        account_size += args.member_of.len() * mem::size_of::<CollectionSignature>()
    }

    create_or_allocate_account_raw(
        *program_id,
        accounts.collection,
        accounts.rent,
        accounts.system,
        accounts.payer,
        account_size,
        &[
            PREFIX.as_bytes(),
            program_id.as_ref(),
            accounts.creator.key.as_ref(),
            args.name.as_bytes(),
            &[bump],
        ],
    )?;

    let authorities: Vec<Pubkey> = Vec::from([accounts.creator.key.clone()]);
    CollectionData {
        name: args.name,
        description: args.description,
        image: args.image,
        creator: accounts.creator.key.clone(),
        authorities,
        advanced: args.advanced,
        max_size: args.max_size,
        members: args.members.clone(),
        member_of: args.member_of.clone(),
    }
    .serialize(&mut *accounts.collection.data.borrow_mut())?;

    Ok(())
}
