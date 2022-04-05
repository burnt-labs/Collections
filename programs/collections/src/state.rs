use anchor_lang::prelude::*;

#[account]
pub struct Collection {
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub mutable: bool,
    pub name: String,
    pub bump: u8,
    pub meta: String,
    pub latest_page_index: u32,
}

impl Collection {
    pub fn space(name: &str, meta: &str) -> usize{
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

const MAX_PAGE_SIZE: usize = 256;

#[account]
pub struct CollectionPage {
    pub collection: Pubkey,
    pub index: u32,
    pub bump: u8,
    pub assets: [Option<Pubkey>; MAX_PAGE_SIZE],
    pub current_index: u8,
}

#[account]
pub struct AssetMapping {
    pub collection: Pubkey,
    pub asset: Pubkey,
    pub bump: u8,
    pub meta: String,
}

impl AssetMapping {
    pub fn space(meta: &str) -> usize{
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
