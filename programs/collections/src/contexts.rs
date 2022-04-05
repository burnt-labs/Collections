use anchor_lang::prelude::*;
use crate::{AssetMapping, Collection, CollectionPage};

const PREFIX: &str = "collections";
const ASSET_PREFIX: &str = "collections-asset";
const PAGE_PREFIX: &str = "collection-page";

#[derive(Accounts)]
#[instruction(name: String, meta: String)]
pub struct InitializeCollection<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = Collection::space(&name, &meta),
        seeds = [PREFIX.as_bytes(), creator.key().as_ref(), name.as_bytes()],
        bump,
    )]
    pub collection: Account<'info, Collection>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(index: u32)]
pub struct AddPage<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [PAGE_PREFIX.as_bytes(), collection.key().as_ref(), &index.to_ne_bytes()],
        bump,
    )]
    pub collection_page: Account<'info, CollectionPage>,
    #[account(
        mut,
        has_one = authority,
        constraint = collection.mutable,
    )]
    pub collection: Account<'info, Collection>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(meta: String)]
pub struct AddAsset<'info> {
    #[account(
        init,
        payer = authority,
        space = AssetMapping::space(&meta),
        seeds = [ASSET_PREFIX.as_bytes(), collection.key().as_ref(), asset.key().as_ref()],
        bump,
    )]
    pub asset_mapping: Account<'info, AssetMapping>,
    #[account(
        mut,
        has_one = authority,
        constraint = collection.mutable,
    )]
    pub collection: Account<'info, Collection>,
    #[account(
        mut,
        has_one = collection,
    )]
    pub collection_page: Account<'info, CollectionPage>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub asset: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveAsset<'info> {
    #[account(
        mut,
        close = authority,
        seeds = [ASSET_PREFIX.as_bytes(), collection.key().as_ref(), asset.key().as_ref()],
        bump = asset_mapping.bump,
    )]
    pub asset_mapping: Account<'info, AssetMapping>,
    #[account(
        mut,
        has_one = authority,
        constraint = collection.mutable,
    )]
    pub collection: Account<'info, Collection>,
    #[account(
        mut,
        has_one = collection,
    )]
    pub collection_page: Account<'info, CollectionPage>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub asset: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = collection.mutable,
    )]
    pub collection: Account<'info, Collection>,
    pub authority: Signer<'info>,
    pub new_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FreezeCollection<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub collection: Account<'info, Collection>,
    pub authority: Signer<'info>,
}
