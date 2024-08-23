use crate::enums::reaction_type::ReactionType;
use crate::structs::chat::Chat;
use crate::structs::message_id::MessageId;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_reaction_updated::MessageReactionUpdated as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,
}
