use borsh::{BorshDeserialize, BorshSerialize};

use crate::processor::add_member_of::AddMemberOfArgs;
use crate::processor::arrange_member::ArrangeMemberArgs;
use crate::processor::freeze_collection::FreezeCollectionArgs;
use crate::processor::remove_member::RemoveMemberArgs;
use crate::processor::{add_members::AddMembersArgs, AddAuthorityArgs, create_collection::CreateCollectionArgs, RemoveAuthorityArgs};

#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CollectionInstruction {
    CreateCollection(CreateCollectionArgs),
    AddMembers(AddMembersArgs),
    RemoveMember(RemoveMemberArgs),
    ArrangeMember(ArrangeMemberArgs),
    AddMemberOf(AddMemberOfArgs),
    FreezeCollection(FreezeCollectionArgs),
    AddAuthority(AddAuthorityArgs),
    RemoveAuthority(RemoveAuthorityArgs),
}
