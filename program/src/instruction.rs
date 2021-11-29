use borsh::{BorshDeserialize, BorshSerialize};

use crate::processor::add_member_of::AddMemberOfArgs;
use crate::processor::arrange_member::ArrangeMemberArgs;
use crate::processor::remove_member::RemoveMemberArgs;
pub use crate::processor::{add_members::AddMembersArgs, create_collection::CreateCollectionArgs};

#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CollectionInstruction {
    CreateCollection(CreateCollectionArgs),
    AddMembers(AddMembersArgs),
    RemoveMember(RemoveMemberArgs),
    ArrangeMember(ArrangeMemberArgs),
    AddMemberOf(AddMemberOfArgs),
}
