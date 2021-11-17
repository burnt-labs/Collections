use borsh::{BorshDeserialize, BorshSerialize};

use crate::processor::add_member_of::AddMemberOfArgs;
use crate::processor::arrange_member::ArrangeMemberArgs;
use crate::processor::remove_member::RemoveMemberArgs;
pub use crate::processor::{add_member::AddMemberArgs, create_collection::CreateCollectionArgs};

#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CollectionInstruction {
    CreateCollection(CreateCollectionArgs),
    AddMember(AddMemberArgs),
    RemoveMember(RemoveMemberArgs),
    ArrangeMember(ArrangeMemberArgs),
    AddMemberOf(AddMemberOfArgs),
}
