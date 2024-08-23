use crate::structs::chat_members::chat_member_administrator::ChatMemberAdministrator;
use crate::structs::chat_members::chat_member_banned::ChatMemberBanned;
use crate::structs::chat_members::chat_member_left::ChatMemberLeft;
use crate::structs::chat_members::chat_member_member::ChatMemberMember;
use crate::structs::chat_members::chat_member_owner::ChatMemberOwner;
use crate::structs::chat_members::chat_member_restricted::ChatMemberRestricted;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::chat_member::ChatMember as Remote;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMember {
    Owner(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Banned(ChatMemberBanned),
}

impl From<Remote> for ChatMember {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Owner(owner) => Self::Owner(owner.into()),
            Remote::Administrator(administrator) => Self::Administrator(administrator.into()),
            Remote::Member(member) => Self::Member(member.into()),
            Remote::Restricted(restricted) => Self::Restricted(restricted.into()),
            Remote::Left(left) => Self::Left(left.into()),
            Remote::Banned(banned) => Self::Banned(banned.into()),
        }
    }
}

impl Default for ChatMember {
    fn default() -> Self {
        Self::Owner(ChatMemberOwner {
            ..Default::default()
        })
    }
}
