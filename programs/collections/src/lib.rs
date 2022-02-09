use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const PREFIX: &str = "collections";

#[program]
pub mod collections {
    use crate::ErrorCode::BumpNotInContext;
    use super::*;

    pub fn initialize_collection(
        ctx: Context<InitializeCollection>,
        name: String,
        meta: String
    ) -> ProgramResult {
        let collection = &mut ctx.accounts.collection;

        collection.authority = ctx.accounts.creator.key();
        collection.creator = ctx.accounts.creator.key();
        collection.name = name;
        collection.meta = meta;
        collection.mutable = true;
        collection.bump = *ctx.bumps.get("collection").ok_or(BumpNotInContext)?;

        Ok(())
    }

    pub fn add_asset(ctx: Context<AddAsset>, meta: String) -> ProgramResult {
        let asset_mapping = &mut ctx.accounts.asset_mapping;
        asset_mapping.asset = ctx.accounts.asset.key();
        asset_mapping.collection = ctx.accounts.collection.key();
        asset_mapping.meta = meta;
        asset_mapping.bump = *ctx.bumps.get("asset_mapping").ok_or(BumpNotInContext)?;

        Ok(())
    }

    pub fn remove_asset(_ctx: Context<RemoveAsset>) -> ProgramResult {
        Ok(())
    }

    pub fn update_authority(ctx: Context<UpdateAuthority>) -> ProgramResult {
        let collection = &mut ctx.accounts.collection;
        collection.authority = ctx.accounts.new_authority.key();

        Ok(())
    }

    pub fn freeze_collection(ctx: Context<FreezeCollection>) -> ProgramResult {
        let collection = &mut ctx.accounts.collection;
        collection.mutable = false;

        Ok(())
    }
}

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
#[instruction(meta: String)]
pub struct AddAsset<'info> {
    #[account(
        init,
        payer = authority,
        space = AssetMapping::space(&meta),
        seeds = [PREFIX.as_bytes(), collection.key().as_ref(), asset.key().as_ref()],
        bump,
    )]
    pub asset_mapping: Account<'info, AssetMapping>,
    #[account(
        mut,
        has_one = authority,
        constraint = collection.mutable,
    )]
    pub collection: Account<'info, Collection>,
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
        seeds = [PREFIX.as_bytes(), collection.key().as_ref(), asset.key().as_ref()],
        bump = asset_mapping.bump,
    )]
    pub asset_mapping: Account<'info, AssetMapping>,
    #[account(
        mut,
        has_one = authority,
        constraint = collection.mutable,
    )]
    pub collection: Account<'info, Collection>,
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
    pub new_authority: AccountInfo<'info>,
}

// Account data definitions

#[account]
pub struct Collection {
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub mutable: bool,
    pub name: String,
    pub bump: u8,
    pub meta: String,
}

impl Collection {
    fn space(name: &str, meta: &str) -> usize{
        // anchor account discriminator
        8 +
        // creator pubkey
        32 +
        // authority pubkey
        32 +
        // mutable boolean
        1 +
        // name String
        4 + name.len() +
        // bump u8
        1 +
        // meta String
        4 + meta.len()
    }
}

#[account]
pub struct AssetMapping {
    pub collection: Pubkey,
    pub asset: Pubkey,
    pub bump: u8,
    pub meta: String,
}

impl AssetMapping {
    fn space(meta: &str) -> usize{
        // anchor account discriminator
        8 +
        // collection pubkey
        32 +
        // asset pubkey
        32 +
        // bump u8
        1 +
        // meta String
        4 + meta.len()
    }
}


// Errors

#[error]
pub enum ErrorCode {
    #[msg("The bump was not found for the account name in this context")]
    BumpNotInContext,
}