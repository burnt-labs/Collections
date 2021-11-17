use borsh::{BorshDeserialize, BorshSerialize};

use crate::processor::{
    add_member::AddMemberArgs, add_member_of::AddMemberOfArgs, arrange_member::ArrangeMemberArgs,
    create_collection::CreateCollectionArgs, remove_member::RemoveMemberArgs,
};

// TODO: Fix the serialize and deserialize errors
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CollectionInstruction {
    CreateCollection(CreateCollectionArgs),
    AddMember(AddMemberArgs),
    RemoveMember(RemoveMemberArgs),
    ArrangeMember(ArrangeMemberArgs),
    AddMemberOf(AddMemberOfArgs),
}
