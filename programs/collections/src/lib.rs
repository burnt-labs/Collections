use anchor_lang::prelude::*;

mod errors;
mod state;
mod contexts;

use errors::*;
use state::*;
use contexts::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod collections {
    use crate::ErrorCode::{BumpNotInContext, PageFull};
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
        collection.latest_page_index = 0;
        collection.bump = *ctx.bumps.get("collection").ok_or(BumpNotInContext)?;

        Ok(())
    }

    pub fn add_page(ctx: Context<AddPage>, index: u32) -> ProgramResult {
        let collection_page = &mut ctx.accounts.collection_page;
        collection_page.collection = ctx.accounts.collection.key();
        collection_page.index = index;
        collection_page.bump = *ctx.bumps.get("collection_page").ok_or(BumpNotInContext)?;

        let collection = &mut ctx.accounts.collection;
        collection.latest_page_index = index;

        Ok(())
    }

    pub fn add_asset(ctx: Context<AddAsset>, meta: String) -> ProgramResult {
        let asset_mapping = &mut ctx.accounts.asset_mapping;
        asset_mapping.asset = ctx.accounts.asset.key();
        asset_mapping.collection = ctx.accounts.collection.key();
        asset_mapping.meta = meta;
        asset_mapping.bump = *ctx.bumps.get("asset_mapping").ok_or(BumpNotInContext)?;

        let collection_page = &mut ctx.accounts.collection_page;
        if collection_page.current_index == 255 {
            Err(PageFull.into());
        }
        collection_page.assets[collection_page.current_index as usize] = Some(ctx.accounts.asset.key());
        collection_page.current_index += 1;

        Ok(())
    }

    pub fn remove_asset(ctx: Context<RemoveAsset>) -> ProgramResult {
        let collection_page = &mut ctx.accounts.collection_page;

        for i in 0..collection_page.assets.len() {
            if collection_page.assets[i] == Some(ctx.accounts.asset.key()) {
                collection_page.assets[i] = None;
            }
        }

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
