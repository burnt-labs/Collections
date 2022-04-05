use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("The bump was not found for the account name in this context")]
    BumpNotInContext,
    #[msg("this page has run out of space")]
    PageFull,
}